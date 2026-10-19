//! Relative branch targets of a compiled function, as local labels.
//!
//! LLVM writes a conditional branch whose target is out of its range as an
//! inverted branch over an unconditional one: `bc 12, 1, .+8` followed by
//! `b .LBB0_9`. The rewrite moves and removes instructions, so a byte
//! offset would no longer name the same instruction. [`TargetLabels`]
//! places a local label before each target that the assembly text fixes
//! and aims the branch at it: the assembler then computes the offset after
//! the rewrite, and the analysis sees the target as a point where control
//! enters. The label does not change the encoding of the original
//! instructions. A branch whose range a rewrite exceeds fails to assemble.
//!
//! A target is fixed when every line between the branch and it is an
//! instruction (4 bytes) or places nothing in the section (labels, blank
//! lines, debug line and call frame annotations). Other relative branches,
//! and every branch inside inline assembly, keep their text, and
//! [`crate::analysis`] refuses the function. `.+4` keeps its text too: it
//! names the next instruction however the rewrite changes the code.

use crate::isa;
use crate::isa::Control;
use crate::line::Line;
use crate::line::parse_integer;

/// Prefix of the labels that [`TargetLabels`] adds; LLVM does not use it.
const LABEL_PREFIX: &str = ".Lxbox360_relative";

const INSTRUCTION_BYTES: i64 = 4;

/// Whether `label` was added by [`TargetLabels`]. A branch names it, so
/// control enters there.
pub fn is_target_label(label: &str) -> bool {
    label.starts_with(LABEL_PREFIX)
}

/// Numbering of the added labels, unique within one assembly file.
#[derive(Debug, Default)]
pub struct TargetLabels {
    next: usize,
}

impl TargetLabels {
    /// `lines` with every fixed relative branch target labeled and every
    /// such branch aimed at its label.
    pub fn apply(&mut self, lines: &[String]) -> Vec<String> {
        let parsed: Vec<Line<'_>> = lines.iter().map(|line| Line::parse(line)).collect();

        // Target line and label name of each resolved branch line.
        let mut branches: Vec<Option<(usize, String)>> = vec![None; lines.len()];
        // Label name placed before each target line.
        let mut labels: Vec<Option<String>> = vec![None; lines.len()];
        let mut inline_asm = false;

        for (index, line) in parsed.iter().enumerate() {
            match line {
                Line::InlineAsmStart => inline_asm = true,
                Line::InlineAsmEnd => inline_asm = false,
                Line::Instruction(_) if inline_asm => {}
                Line::Instruction(instruction) => {
                    let Some(offset) = relative_offset(instruction) else {
                        continue;
                    };
                    let Some(target) = resolve(&parsed, index, offset) else {
                        continue;
                    };

                    let label = labels[target]
                        .get_or_insert_with(|| {
                            let label = format!("{LABEL_PREFIX}{}", self.next);
                            self.next += 1;
                            label
                        })
                        .clone();
                    branches[index] = Some((target, label));
                }
                _ => {}
            }
        }

        let mut output = Vec::with_capacity(lines.len());
        for (index, line) in lines.iter().enumerate() {
            if let Some(label) = &labels[index] {
                output.push(format!("{label}:"));
            }

            match &branches[index] {
                Some((_, label)) => output.push(retarget(line, label)),
                None => output.push(line.clone()),
            }
        }

        output
    }
}

/// Byte offset of a relative jump other than `.+4`. Absolute forms
/// (`ba`, `bca`, `beqa`, ...) name an address, not an offset.
fn relative_offset(instruction: &crate::line::Instruction<'_>) -> Option<i64> {
    let Control::Jump { target, .. } = isa::control(instruction) else {
        return None;
    };

    if instruction
        .mnemonic
        .trim_end_matches(['+', '-'])
        .ends_with('a')
    {
        return None;
    }

    let offset = match target.strip_prefix('.')? {
        "" => 0,
        rest => parse_integer(rest.strip_prefix('+').unwrap_or(rest))
            .filter(|_| rest.starts_with(['+', '-']))?,
    };

    (offset != INSTRUCTION_BYTES).then_some(offset)
}

/// Bytes a line places in the section; `None` when the text does not fix it.
fn size(line: &Line<'_>) -> Option<i64> {
    match line {
        Line::Instruction(_) => Some(INSTRUCTION_BYTES),
        Line::Label(_) | Line::Blank | Line::Annotation(_) => Some(0),
        Line::Directive(_) | Line::Statements(_) | Line::InlineAsmStart | Line::InlineAsmEnd => {
            None
        }
    }
}

/// Line of the instruction at `offset` bytes from the branch at `branch`.
fn resolve(lines: &[Line<'_>], branch: usize, offset: i64) -> Option<usize> {
    if offset % INSTRUCTION_BYTES != 0 {
        return None;
    }

    if offset >= 0 {
        let mut distance = 0;
        for (index, line) in lines.iter().enumerate().skip(branch) {
            if matches!(line, Line::Instruction(_)) && distance == offset {
                return Some(index);
            }

            distance += size(line)?;
            if distance > offset {
                return None;
            }
        }
    } else {
        let mut distance = 0;
        for index in (0..branch).rev() {
            distance -= size(&lines[index])?;
            if matches!(lines[index], Line::Instruction(_)) && distance == offset {
                return Some(index);
            }

            if distance < offset {
                return None;
            }
        }
    }

    None
}

/// `line` with its last operand, the relative target, replaced by `label`.
fn retarget(line: &str, label: &str) -> String {
    let code_end = line.find('#').unwrap_or(line.len());
    let code = line[..code_end].trim_end();
    let operand_start = code
        .rfind([',', ' ', '\t'])
        .map_or(0, |position| position + 1);

    format!("{}{label}{}", &code[..operand_start], &line[code.len()..])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn labeled(text: &str) -> String {
        let lines: Vec<String> = text.lines().map(str::to_owned).collect();
        TargetLabels::default().apply(&lines).join("\n")
    }

    #[test]
    fn labels_target_of_relaxed_branch() {
        let text = "\tbc 12, 1, .+8
\tb .LBB0_9
.Ltmp3:
\t.loc\t73 92 17
\tlwz 3, 0(28)";
        assert_eq!(
            labeled(text),
            "\tbc 12, 1, .Lxbox360_relative0
\tb .LBB0_9
.Ltmp3:
\t.loc\t73 92 17
.Lxbox360_relative0:
\tlwz 3, 0(28)"
        );
    }

    #[test]
    fn labels_backward_and_self_targets_once() {
        let text = "\tli 3, 0
\taddi 3, 3, 1
\tbdnz+ .-4
\tbne 0, .-8
\tb .";
        assert_eq!(
            labeled(text),
            "\tli 3, 0
.Lxbox360_relative0:
\taddi 3, 3, 1
\tbdnz+ .Lxbox360_relative0
\tbne 0, .Lxbox360_relative0
.Lxbox360_relative1:
\tb .Lxbox360_relative1"
        );
    }

    #[test]
    fn keeps_unfixed_targets_and_next_instruction_branches() {
        let text = "\tbne 0, .+12
\t.p2align\t4
\tnop
\tnop
\tbne- 7, .+4
\tisync
\tbeq 0, .+6
\tnop
\tba .+8
\tnop
\tnop
\t#APP
\tbne 0, .+8
\tnop
\tnop
\t#NO_APP
\tbne 0, .+64
\tnop";
        assert_eq!(labeled(text), text);
    }

    #[test]
    fn keeps_trailing_comment() {
        assert_eq!(
            labeled("\tbeq+\t0, .+8 # skip\n\tb .LBB0_2\n\tnop"),
            "\tbeq+\t0, .Lxbox360_relative0 # skip\n\tb .LBB0_2\n.Lxbox360_relative0:\n\tnop"
        );
    }
}
