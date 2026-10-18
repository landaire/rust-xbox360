//! Local labels at which control can enter other than by falling through.
//!
//! A straight-line block ends at every label that an instruction, a jump
//! table, or other data in a loaded section names. With debug information,
//! LLVM also places local labels inside blocks to mark line and scope
//! boundaries (`.Ltmp*`); only the debug sections name those, so they do not
//! end a block. Local labels (`.L` prefix) never reach the object's symbol
//! table, so every reference to one is in the same assembly file.

use std::collections::HashSet;
use std::io;
use std::io::BufRead;

use crate::line::Line;

/// Local labels that the assembly names outside the debug sections.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LabelReferences {
    referenced: HashSet<String>,
}

impl LabelReferences {
    /// Reads the whole assembly file once.
    pub fn scan(input: impl BufRead) -> io::Result<Self> {
        let mut referenced = HashSet::new();
        let mut sections = Sections::default();

        for line in input.lines() {
            let line = line?;
            let parsed = Line::parse(&line);

            if let Line::Directive(directive) = parsed {
                sections.apply(directive);
            }

            if sections.current.debug || matches!(parsed, Line::Label(_)) {
                continue;
            }

            // Comments and strings are scanned too: an extra name only
            // keeps a block boundary.
            referenced.extend(local_labels(&line).map(str::to_owned));
        }

        Ok(Self { referenced })
    }

    /// Whether control may enter at `label` other than from the line
    /// before it. Labels other than local labels may be named by other
    /// objects.
    pub fn may_be_entered(&self, label: &str) -> bool {
        !label.starts_with(".L") || self.referenced.contains(label)
    }
}

/// Names that start with `.L` in `text`.
fn local_labels(text: &str) -> impl Iterator<Item = &str> {
    let is_name = |character: char| character.is_ascii_alphanumeric() || "_.$".contains(character);
    let mut rest = text;

    std::iter::from_fn(move || {
        loop {
            let start = rest.find(is_name)?;
            let after = &rest[start..];
            let length = after
                .find(|character| !is_name(character))
                .unwrap_or(after.len());
            let name = &after[..length];
            rest = &after[length..];

            if name.starts_with(".L") {
                return Some(name);
            }
        }
    })
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Section {
    /// A `.debug_*` section, which is not loaded.
    debug: bool,
}

/// Section state of the LLVM ELF assembly parser: the current section,
/// the one `.previous` returns to, and the `.pushsection` stack.
#[derive(Debug, Default)]
struct Sections {
    current: Section,
    previous: Section,
    stack: Vec<(Section, Section)>,
}

impl Sections {
    fn apply(&mut self, directive: &str) {
        let name = Line::directive_name(directive);
        let operands = directive[name.len()..].trim();

        match name {
            ".section" => self.switch(section_named(operands)),
            ".pushsection" => {
                self.stack.push((self.current, self.previous));
                self.switch(section_named(operands));
            }
            ".popsection" => {
                // An unmatched pop is an assembler error; a loaded section
                // keeps every later reference.
                (self.current, self.previous) = self.stack.pop().unwrap_or_default();
            }
            ".previous" => std::mem::swap(&mut self.current, &mut self.previous),
            ".text" | ".data" | ".bss" | ".rodata" | ".tdata" | ".tbss" => {
                self.switch(Section { debug: false })
            }
            _ => {}
        }
    }

    fn switch(&mut self, section: Section) {
        self.previous = self.current;
        self.current = section;
    }
}

/// `.section NAME[, FLAGS...]` operands.
fn section_named(operands: &str) -> Section {
    let name = operands
        .split(|character: char| character == ',' || character.is_whitespace())
        .next()
        .unwrap_or(operands)
        .trim_matches('"');

    Section {
        debug: name.starts_with(".debug_"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scan(text: &str) -> LabelReferences {
        LabelReferences::scan(text.as_bytes()).expect("scan")
    }

    #[test]
    fn ignores_references_from_debug_sections() {
        let references = scan(
            "\t.section\t.text.f,\"ax\",@progbits
f:
.Ltmp0:
\tbc 12, 2, .LBB0_2
.LBB0_2:
\tblr
\t.section\t.debug_info,\"\",@progbits
\t.long\t.Ltmp0
\t.long\t.LBB0_9
\t.section\t.rodata,\"a\",@progbits
\t.long\t.LBB0_3-.LJTI0_0
",
        );

        assert!(!references.may_be_entered(".Ltmp0"));
        assert!(!references.may_be_entered(".LBB0_9"));
        assert!(references.may_be_entered(".LBB0_2"));
        assert!(references.may_be_entered(".LBB0_3"));
        assert!(references.may_be_entered("f"));
    }

    #[test]
    fn follows_section_stack() {
        let references = scan(
            "\t.section\t.debug_line,\"\",@progbits
\t.pushsection .text.g,\"ax\"
\tb .LBB1_1
\t.popsection
\t.long .Ltmp1
\t.text
\t.long .Ltmp2
\t.previous
\t.long .Ltmp3
\t.previous
\t.long .Ltmp4
",
        );

        assert!(references.may_be_entered(".LBB1_1"));
        assert!(!references.may_be_entered(".Ltmp1"));
        assert!(references.may_be_entered(".Ltmp2"));
        assert!(!references.may_be_entered(".Ltmp3"));
        assert!(references.may_be_entered(".Ltmp4"));
    }
}
