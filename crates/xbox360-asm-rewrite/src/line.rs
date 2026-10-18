//! Line model of LLVM PowerPC assembly output.

/// One line of assembly.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Line<'a> {
    Label(&'a str),
    Instruction(Instruction<'a>),
    Directive(&'a str),
    /// Directive that places nothing in the section and has no effect on
    /// the code: debug line information (`.loc`, `.file`) and call frame
    /// information (`.cfi_*`).
    Annotation(&'a str),
    /// Several statements joined by `;`; never analyzed.
    Statements(&'a str),
    /// `#APP`: start of an inline assembly block.
    InlineAsmStart,
    /// `#NO_APP`: end of an inline assembly block.
    InlineAsmEnd,
    /// Empty line or comment.
    Blank,
}

impl<'a> Line<'a> {
    pub fn parse(raw: &'a str) -> Self {
        let text = raw.trim_end();
        let trimmed = text.trim_start();
        if trimmed.is_empty() {
            return Self::Blank;
        }

        if let Some(comment) = trimmed.strip_prefix('#') {
            return match comment.trim() {
                "APP" => Self::InlineAsmStart,
                "NO_APP" => Self::InlineAsmEnd,
                _ => Self::Blank,
            };
        }

        let indented = trimmed.len() != text.len();
        if trimmed.starts_with('.') && indented {
            return Self::directive(trimmed);
        }

        let code = strip_comment(trimmed);
        if let Some(label) = code.strip_suffix(':')
            && !label.contains(char::is_whitespace)
        {
            return Self::Label(label);
        }

        if !indented {
            return Self::directive(code);
        }

        if code.contains(';') {
            return Self::Statements(code);
        }

        Self::Instruction(Instruction::parse(code))
    }

    fn directive(text: &'a str) -> Self {
        if is_annotation(text) {
            Self::Annotation(text)
        } else {
            Self::Directive(text)
        }
    }

    /// Name of a directive or annotation, such as `.cfi_offset`.
    pub fn directive_name(text: &str) -> &str {
        text.split_whitespace().next().unwrap_or(text)
    }
}

fn is_annotation(text: &str) -> bool {
    let name = Line::directive_name(text);
    name == ".loc" || name == ".file" || name.starts_with(".cfi_")
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instruction<'a> {
    pub mnemonic: &'a str,
    pub operands: Vec<Operand<'a>>,
}

impl<'a> Instruction<'a> {
    pub fn parse(code: &'a str) -> Self {
        let (mnemonic, rest) = match code.find(char::is_whitespace) {
            Some(split) => (&code[..split], code[split..].trim()),
            None => (code, ""),
        };

        let operands = if rest.is_empty() {
            Vec::new()
        } else {
            split_operands(rest).map(Operand::parse).collect()
        };

        Self { mnemonic, operands }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operand<'a> {
    /// Register number or immediate; the mnemonic decides which.
    Integer(i64),
    /// `displacement(base)`.
    Memory {
        displacement: Displacement<'a>,
        base: i64,
    },
    Expression(&'a str),
}

impl<'a> Operand<'a> {
    pub fn parse(text: &'a str) -> Self {
        if let Some(value) = parse_integer(text) {
            return Self::Integer(value);
        }

        if let Some(inner) = text.strip_suffix(')')
            && let Some(open) = inner.rfind('(')
            && let Some(base) = parse_integer(&inner[open + 1..])
        {
            let displacement = inner[..open].trim();
            if !displacement.is_empty() {
                let displacement = match parse_integer(displacement) {
                    Some(value) => Displacement::Integer(value),
                    None => Displacement::Expression(displacement),
                };

                return Self::Memory { displacement, base };
            }
        }

        Self::Expression(text)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Displacement<'a> {
    Integer(i64),
    Expression(&'a str),
}

fn strip_comment(code: &str) -> &str {
    match code.find('#') {
        Some(position) => code[..position].trim_end(),
        None => code,
    }
}

/// Splits at commas outside parentheses.
fn split_operands(text: &str) -> impl Iterator<Item = &str> {
    let mut depth = 0i32;
    let mut start = 0;
    let mut pieces = Vec::new();

    for (index, character) in text.char_indices() {
        match character {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => {
                pieces.push(text[start..index].trim());
                start = index + 1;
            }
            // Part of the current operand.
            _ => {}
        }
    }

    pieces.push(text[start..].trim());
    pieces.into_iter()
}

pub fn parse_integer(text: &str) -> Option<i64> {
    let (negative, digits) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text),
    };

    let magnitude = match digits.strip_prefix("0x") {
        Some(hex) if !hex.is_empty() && hex.bytes().all(|byte| byte.is_ascii_hexdigit()) => {
            i64::from_str_radix(hex, 16).ok()?
        }
        Some(_) => return None,
        None if !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit()) => {
            digits.parse().ok()?
        }
        None => return None,
    };

    Some(if negative { -magnitude } else { magnitude })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_lines() {
        assert_eq!(Line::parse(".LBB3_2:"), Line::Label(".LBB3_2"));
        assert_eq!(Line::parse("\t.p2align\t2"), Line::Directive(".p2align\t2"));
        assert_eq!(
            Line::parse("\t.loc\t2 2234 9 prologue_end"),
            Line::Annotation(".loc\t2 2234 9 prologue_end")
        );
        assert_eq!(
            Line::parse("\t.cfi_offset lr, 4"),
            Line::Annotation(".cfi_offset lr, 4")
        );
        assert_eq!(
            Line::parse("\t.file\t30 \"sysroot/library/alloc/src/vec\" \"mod.rs\""),
            Line::Annotation(".file\t30 \"sysroot/library/alloc/src/vec\" \"mod.rs\"")
        );
        assert_eq!(
            Line::parse("\t.local_label_prefix"),
            Line::Directive(".local_label_prefix")
        );
        assert_eq!(Line::parse("\t#APP"), Line::InlineAsmStart);
        assert_eq!(Line::parse("\t#NO_APP"), Line::InlineAsmEnd);
        assert_eq!(Line::parse("\t# %bb.1"), Line::Blank);
        assert_eq!(Line::parse(""), Line::Blank);
        assert_eq!(Line::parse("\t1:"), Line::Label("1"));
        assert_eq!(
            Line::parse("\tcmpwi 3, 0; beq .+16"),
            Line::Statements("cmpwi 3, 0; beq .+16")
        );
        assert_eq!(
            Line::parse("\tstw 30, 8(1)"),
            Line::Instruction(Instruction {
                mnemonic: "stw",
                operands: vec![
                    Operand::Integer(30),
                    Operand::Memory {
                        displacement: Displacement::Integer(8),
                        base: 1
                    }
                ],
            })
        );
    }

    #[test]
    fn parses_symbolic_operands() {
        let Line::Instruction(instruction) = Line::parse("\tla 21, .L_MergedGlobals.1@l(3)") else {
            panic!("not an instruction");
        };
        assert_eq!(
            instruction.operands,
            vec![
                Operand::Integer(21),
                Operand::Memory {
                    displacement: Displacement::Expression(".L_MergedGlobals.1@l"),
                    base: 3
                }
            ]
        );
        let Line::Instruction(instruction) = Line::parse("\tbc 4, 1, .LBB0_2") else {
            panic!("not an instruction");
        };
        assert_eq!(instruction.operands[2], Operand::Expression(".LBB0_2"));
        let Line::Instruction(instruction) = Line::parse("\tmr\t3, 4 # copy") else {
            panic!("not an instruction");
        };
        assert_eq!(instruction.mnemonic, "mr");
        assert_eq!(
            instruction.operands,
            vec![Operand::Integer(3), Operand::Integer(4)]
        );
    }
}
