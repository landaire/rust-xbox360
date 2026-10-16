//! Reapplies the rewrite rules to `llvm-objdump -d --no-show-raw-insn`
//! output of the linked image and reports every save or restore sequence
//! that the rules would still rewrite.
//!
//! Only functions listed in the scope file written by [`crate::rewrite`]
//! are checked. Functions with inline assembly are skipped: the rewriter
//! treats inline assembly as opaque, which the disassembly cannot show.

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fmt;
use std::io::BufRead;
use std::io::{
    self,
};

use thiserror::Error;

use crate::analysis::Function;
use crate::line::Line;
use crate::line::parse_integer;
use crate::rewrite::SiteCounts;

#[derive(Debug, Error)]
pub enum Error {
    #[error("reading the scope file")]
    ReadScope(#[source] io::Error),
    #[error("scope line {line} is not `name<TAB>inline-asm|plain`")]
    MalformedScope { line: usize },
    #[error("reading the disassembly")]
    ReadDisassembly(#[source] io::Error),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScopeKind {
    Plain,
    InlineAsm,
}

/// Compiled functions of the rewritten assembly.
pub fn read_scope(input: impl BufRead) -> Result<HashMap<String, ScopeKind>, Error> {
    let mut scope = HashMap::new();
    for (number, line) in input.lines().enumerate() {
        let line = line.map_err(Error::ReadScope)?;
        let (name, kind) = line
            .split_once('\t')
            .ok_or(Error::MalformedScope { line: number + 1 })?;
        let kind = match kind {
            "plain" => ScopeKind::Plain,
            "inline-asm" => ScopeKind::InlineAsm,
            _ => return Err(Error::MalformedScope { line: number + 1 }),
        };
        scope.insert(name.to_owned(), kind);
    }
    Ok(scope)
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AuditReport {
    pub functions_checked: usize,
    pub functions_with_inline_asm: usize,
    /// "rewritten" counts sites the rules still accept.
    pub prologues: SiteCounts,
    pub epilogues: SiteCounts,
    /// Functions holding a site the rules still accept.
    pub rewritable: BTreeSet<String>,
    /// Branches to the helper routines anywhere in the image.
    pub helper_branches: usize,
}

impl AuditReport {
    pub fn passed(&self) -> bool {
        self.rewritable.is_empty()
    }
}

impl fmt::Display for AuditReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "functions checked: {} ({} with inline assembly skipped)",
            self.functions_checked, self.functions_with_inline_asm
        )?;
        writeln!(f, "helper branches: {}", self.helper_branches)?;
        for (name, counts) in [
            ("prologues", &self.prologues),
            ("epilogues", &self.epilogues),
        ] {
            writeln!(f, "{name}: {} rewritable", counts.rewritten)?;
            for (kind, count) in &counts.refused {
                writeln!(f, "  refused {kind}: {count}")?;
            }
        }
        for name in &self.rewritable {
            writeln!(f, "rewritable: {name}")?;
        }
        Ok(())
    }
}

struct Disassembled {
    name: String,
    instructions: Vec<(u64, String)>,
}

pub fn audit(
    disassembly: impl BufRead,
    scope: &HashMap<String, ScopeKind>,
) -> Result<AuditReport, Error> {
    let mut report = AuditReport::default();
    let mut current: Option<Disassembled> = None;
    for line in disassembly.lines() {
        let line = line.map_err(Error::ReadDisassembly)?;
        if let Some(name) = function_header(&line) {
            if let Some(function) = current.take() {
                check(function, scope, &mut report);
            }
            current = Some(Disassembled {
                name: name.to_owned(),
                instructions: Vec::new(),
            });
        } else if let Some((address, text)) = instruction_line(&line) {
            if text.contains("<__xbox360_") {
                report.helper_branches += 1;
            }
            if let Some(function) = &mut current {
                function.instructions.push((address, text.to_owned()));
            }
        }
    }
    if let Some(function) = current {
        check(function, scope, &mut report);
    }
    Ok(report)
}

/// `91f0100c <CreateEventA>:`
fn function_header(line: &str) -> Option<&str> {
    let (address, rest) = line.split_once(' ')?;
    u64::from_str_radix(address, 16).ok()?;
    rest.strip_prefix('<')?.strip_suffix(">:")
}

/// `91f0100c:      \tstwu 1, -80(1)`
fn instruction_line(line: &str) -> Option<(u64, &str)> {
    let (address, rest) = line.trim_start().split_once(':')?;
    let address = u64::from_str_radix(address, 16).ok()?;
    let (_, text) = rest.split_once('\t')?;
    Some((address, text.trim()))
}

/// Branch target address of a disassembled branch.
fn branch_target(text: &str) -> Option<(usize, u64)> {
    if !text.starts_with('b') {
        return None;
    }
    let start = text.find("0x")?;
    let end = text[start..]
        .find(char::is_whitespace)
        .map_or(text.len(), |length| start + length);
    let address = parse_integer(&text[start..end])?;
    Some((start, u64::try_from(address).ok()?))
}

fn local_label(address: u64) -> String {
    format!(".La{address:x}")
}

fn check(function: Disassembled, scope: &HashMap<String, ScopeKind>, report: &mut AuditReport) {
    match scope.get(&function.name) {
        None => return,
        Some(ScopeKind::InlineAsm) => {
            report.functions_with_inline_asm += 1;
            return;
        }
        Some(ScopeKind::Plain) => report.functions_checked += 1,
    }
    let Some((&(start, _), &(last, _))) = function
        .instructions
        .first()
        .zip(function.instructions.last())
    else {
        return;
    };
    let contains = |address: u64| (start..=last).contains(&address);
    let targets: BTreeSet<u64> = function
        .instructions
        .iter()
        .filter_map(|(_, text)| branch_target(text))
        .map(|(_, address)| address)
        .filter(|address| contains(*address))
        .collect();
    let mut text = Vec::new();
    for (address, instruction) in &function.instructions {
        if targets.contains(address) {
            text.push(format!("{}:", local_label(*address)));
        }
        let rewritten = match branch_target(instruction) {
            Some((position, target)) if contains(target) => {
                format!("\t{}{}", &instruction[..position], local_label(target))
            }
            Some((position, target)) => {
                format!("\t{}external_{target:x}", &instruction[..position])
            }
            None => format!("\t{instruction}"),
        };
        text.push(rewritten);
    }
    let parsed = Function::new(text.iter().map(|line| Line::parse(line)).collect());
    let analysis = parsed.analyze();
    let mut rewritable = false;
    if let Some(prologue) = &analysis.prologue {
        report.prologues.record(prologue);
        rewritable |= prologue.is_ok();
    }
    for epilogue in &analysis.epilogues {
        report.epilogues.record(epilogue);
        rewritable |= epilogue.is_ok();
    }
    if rewritable {
        report.rewritable.insert(function.name);
    }
}
