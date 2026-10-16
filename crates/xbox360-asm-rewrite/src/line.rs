//! Line model of LLVM PowerPC assembly output.

/// One line of assembly.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Line<'a> {
    Label(&'a str),
    Instruction(Instruction<'a>),
    Directive(&'a str),
    /// Several statements joined by `;`; never analyzed.
    Statements(&'a str),
    /// `#APP`: start of an inline assembly block.
    InlineAsmStart,
    /// `#NO_APP`: end of an inline assembly block.
    InlineAsmEnd,
    /// Empty line or comment.
    Blank,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instruction<'a> {
    pub mnemonic: &'a str,
    pub operands: Vec<Operand<'a>>,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Displacement<'a> {
    Integer(i64),
    Expression(&'a str),
}

impl<'a> Line<'a> {
    pub fn parse(raw: &'a str) -> Self {
        let text = raw.trim_end();
        let trimmed = text.trim_start();
        if trimmed.is_empty() {
            return Line::Blank;
        }
        if let Some(comment) = trimmed.strip_prefix('#') {
            return match comment.trim() {
                "APP" => Line::InlineAsmStart,
                "NO_APP" => Line::InlineAsmEnd,
                _ => Line::Blank,
            };
        }
        let indented = trimmed.len() != text.len();
        if trimmed.starts_with('.') && indented {
            return Line::Directive(trimmed);
        }
        let code = strip_comment(trimmed);
        if let Some(label) = code.strip_suffix(':')
            && !label.contains(char::is_whitespace)
        {
            return Line::Label(label);
        }
        if !indented {
            return Line::Directive(code);
        }
        if code.contains(';') {
            return Line::Statements(code);
        }
        Line::Instruction(Instruction::parse(code))
    }
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
        Instruction { mnemonic, operands }
    }
}

impl<'a> Operand<'a> {
    pub fn parse(text: &'a str) -> Self {
        if let Some(value) = parse_integer(text) {
            return Operand::Integer(value);
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
                return Operand::Memory { displacement, base };
            }
        }
        Operand::Expression(text)
    }
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
        Some(hex) if !hex.is_empty() && hex.bytes().all(|b| b.is_ascii_hexdigit()) => {
            i64::from_str_radix(hex, 16).ok()?
        }
        Some(_) => return None,
        None if !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()) => {
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
