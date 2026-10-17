//! Streaming rewrite of an assembly file, one compiled function at a time.
//!
//! Compiled functions are the regions between `.Lfunc_beginN:` and
//! `.Lfunc_endN:`. Module-level assembly, including naked functions, has no
//! such labels and passes through unchanged.

use std::collections::BTreeMap;
use std::fmt;
use std::io::BufRead;
use std::io::Write;

use crate::analysis::Function;
use crate::analysis::FunctionAnalysis;
use crate::analysis::Refusal;
use crate::helpers;
use crate::helpers::HelperUse;
use crate::line::Line;

mod error;

pub use self::error::Error;

/// Rewritten and refused sites of one kind.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SiteCounts {
    pub rewritten: usize,
    pub refused: BTreeMap<&'static str, usize>,
}

impl SiteCounts {
    pub fn record<T>(&mut self, result: &Result<T, Refusal>) {
        match result {
            Ok(_) => self.rewritten += 1,
            Err(refusal) => *self.refused.entry(refusal.kind()).or_default() += 1,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Report {
    pub functions: usize,
    pub functions_with_inline_asm: usize,
    pub prologues: SiteCounts,
    pub epilogues: SiteCounts,
    /// Instructions removed from and added to function bodies.
    pub removed_instructions: usize,
    pub added_instructions: usize,
    /// Instructions in the emitted helper routines.
    pub helper_instructions: usize,
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "functions: {} ({} with inline assembly)",
            self.functions, self.functions_with_inline_asm
        )?;

        for (name, counts) in [
            ("prologues", &self.prologues),
            ("epilogues", &self.epilogues),
        ] {
            writeln!(f, "{name}: {} rewritten", counts.rewritten)?;

            for (kind, count) in &counts.refused {
                writeln!(f, "  refused {kind}: {count}")?;
            }
        }

        let net = self.removed_instructions as i64
            - self.added_instructions as i64
            - self.helper_instructions as i64;
        write!(
            f,
            "instructions: {} removed, {} added, {} in helpers, {} bytes saved",
            self.removed_instructions,
            self.added_instructions,
            self.helper_instructions,
            net * 4
        )
    }
}

/// Side outputs of [`rewrite`].
pub struct Listings<S, R> {
    /// `name<TAB>inline-asm|plain` per compiled function, for
    /// [`crate::audit`].
    pub scope: S,
    /// `name<TAB>prologue|epilogue<TAB>reason` per refused site.
    pub refusals: R,
}

/// Function region being collected.
struct OpenFunction {
    name: String,
    end_label: String,
    lines: Vec<String>,
}

/// Replacement text for one line.
#[derive(Clone, Debug, Default)]
struct Edit {
    before: Vec<String>,
    replacement: Option<Vec<String>>,
    removed: bool,
}

/// Rewrites `input` into `output`.
pub fn rewrite(
    input: impl BufRead,
    mut output: impl Write,
    mut listings: Listings<impl Write, impl Write>,
) -> Result<Report, Error> {
    let mut report = Report::default();
    let mut helpers = HelperUse::default();
    let mut symbol: Option<String> = None;
    let mut open: Option<OpenFunction> = None;

    for line in input.lines() {
        let line = line.map_err(Error::Read)?;
        if let Some(function) = &mut open {
            if line.strip_suffix(':') == Some(function.end_label.as_str()) {
                let function = open.take().expect("open function");
                emit_function(
                    &function,
                    &mut output,
                    &mut listings,
                    &mut report,
                    &mut helpers,
                )?;
                writeln!(output, "{line}").map_err(Error::Write)?;
            } else {
                function.lines.push(line);
            }

            continue;
        }

        match Line::parse(&line) {
            Line::Label(label) => {
                if let Some(suffix) = label.strip_prefix(".Lfunc_begin") {
                    open = Some(OpenFunction {
                        name: symbol.take().unwrap_or_else(|| label.to_owned()),
                        end_label: format!(".Lfunc_end{suffix}"),
                        lines: Vec::new(),
                    });
                } else {
                    symbol = Some(label.to_owned());
                }
            }
            // Labels may be followed by blank lines before `.Lfunc_begin`.
            Line::Blank => {}
            _ => symbol = None,
        }

        writeln!(output, "{line}").map_err(Error::Write)?;
    }

    if let Some(function) = open {
        return Err(Error::UnterminatedFunction {
            name: function.name,
            end_label: function.end_label,
        });
    }

    helpers.emit(&mut output).map_err(Error::Write)?;
    report.helper_instructions = helpers.instruction_count();

    output.flush().map_err(Error::Write)?;
    listings.scope.flush().map_err(Error::Write)?;
    listings.refusals.flush().map_err(Error::Write)?;

    Ok(report)
}

fn emit_function(
    function: &OpenFunction,
    output: &mut impl Write,
    listings: &mut Listings<impl Write, impl Write>,
    report: &mut Report,
    helpers: &mut HelperUse,
) -> Result<(), Error> {
    let parsed = Function::new(
        function
            .lines
            .iter()
            .map(|line| Line::parse(line))
            .collect(),
    );
    let analysis = parsed.analyze();

    report.functions += 1;
    report.functions_with_inline_asm += usize::from(analysis.inline_asm);

    let kind = if analysis.inline_asm {
        "inline-asm"
    } else {
        "plain"
    };
    writeln!(listings.scope, "{}\t{kind}", function.name).map_err(Error::Write)?;

    let prologue = analysis
        .prologue
        .iter()
        .map(|result| ("prologue", result.as_ref().err()));
    let epilogues = analysis
        .epilogues
        .iter()
        .map(|result| ("epilogue", result.as_ref().err()));

    for (site, refusal) in prologue.chain(epilogues) {
        if let Some(refusal) = refusal {
            writeln!(listings.refusals, "{}\t{site}\t{refusal}", function.name)
                .map_err(Error::Write)?;
        }
    }

    let edits = plan_edits(&analysis, function.lines.len(), report, helpers);

    for (line, edit) in function.lines.iter().zip(&edits) {
        for text in &edit.before {
            writeln!(output, "{text}").map_err(Error::Write)?;
        }

        if let Some(replacement) = &edit.replacement {
            for text in replacement {
                writeln!(output, "{text}").map_err(Error::Write)?;
            }
        } else if !edit.removed {
            writeln!(output, "{line}").map_err(Error::Write)?;
        }
    }

    Ok(())
}

fn plan_edits(
    analysis: &FunctionAnalysis,
    line_count: usize,
    report: &mut Report,
    helpers: &mut HelperUse,
) -> Vec<Edit> {
    let mut edits = vec![Edit::default(); line_count];

    if let Some(prologue) = &analysis.prologue {
        report.prologues.record(prologue);

        if let Ok(prologue) = prologue {
            let mut inserted = Vec::new();

            if prologue.moved_link_read.is_some() {
                inserted.push("\tmflr 0".to_owned());
            }

            inserted.push(format!("\tbl {}", helpers::save_name(prologue.range)));

            report.added_instructions += inserted.len();
            report.removed_instructions += prologue.removed.len();
            edits[prologue.allocation].before = inserted;

            for index in &prologue.removed {
                edits[*index].removed = true;
            }

            helpers.record_save(prologue.range);
        }
    }

    for epilogue in &analysis.epilogues {
        report.epilogues.record(epilogue);
        let Ok(epilogue) = epilogue else { continue };

        for index in &epilogue.removed {
            edits[*index].removed = true;
        }

        edits[epilogue.ret].replacement = Some(vec![
            format!("\taddi 1, 1, {}", epilogue.frame_size.bytes()),
            format!("\tb {}", helpers::restore_name(epilogue.range)),
        ]);
        helpers.record_restore(epilogue.range);
        report.removed_instructions += epilogue.removed.len() + 1;
        report.added_instructions += 2;
    }

    edits
}
