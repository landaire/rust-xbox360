//! Register, memory, link register, and control effects of PowerPC
//! instructions as printed by LLVM.
//!
//! The GPR effect model covers the instructions that LLVM schedules into
//! prologues and epilogues. Instructions outside it have no model, which
//! every rule treats as a reason to refuse.

use std::fmt;

use crate::line::Displacement;
use crate::line::Instruction;
use crate::line::Operand;

/// General-purpose register number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Gpr(u8);

impl Gpr {
    pub const R0: Gpr = Gpr(0);
    pub const R1: Gpr = Gpr(1);
    pub const R30: Gpr = Gpr(30);
    pub const R31: Gpr = Gpr(31);
    /// First callee-saved register in the SVR4 and Xbox 360 ABIs.
    pub const FIRST_CALLEE_SAVED: Gpr = Gpr(14);

    pub fn new(number: i64) -> Option<Gpr> {
        u8::try_from(number).ok().filter(|n| *n < 32).map(Gpr)
    }

    pub fn number(self) -> u8 {
        self.0
    }

    pub fn is_callee_saved(self) -> bool {
        self >= Self::FIRST_CALLEE_SAVED
    }
}

impl fmt::Display for Gpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "r{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GprSet(u32);

impl GprSet {
    pub fn insert(&mut self, gpr: Gpr) {
        self.0 |= 1 << gpr.0;
    }

    pub fn contains(self, gpr: Gpr) -> bool {
        self.0 & (1 << gpr.0) != 0
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn iter(self) -> impl Iterator<Item = Gpr> {
        (0..32u8).filter(move |n| self.0 & (1 << n) != 0).map(Gpr)
    }

    pub fn first(self) -> Option<Gpr> {
        (self.0 != 0).then(|| Gpr(self.0.trailing_zeros() as u8))
    }

    pub fn last(self) -> Option<Gpr> {
        (self.0 != 0).then(|| Gpr(31 - self.0.leading_zeros() as u8))
    }

    pub fn len(self) -> u32 {
        self.0.count_ones()
    }
}

impl FromIterator<Gpr> for GprSet {
    fn from_iter<I: IntoIterator<Item = Gpr>>(iter: I) -> Self {
        let mut set = GprSet::default();
        for gpr in iter {
            set.insert(gpr);
        }
        set
    }
}

impl fmt::Display for GprSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, gpr) in self.iter().enumerate() {
            if index > 0 {
                f.write_str(",")?;
            }
            write!(f, "{gpr}")?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Access {
    Load,
    Store,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Address {
    /// `offset(base)`; `offset` is `None` for a symbolic displacement.
    Displacement { base: Gpr, offset: Option<i64> },
    /// `base + index`.
    Indexed { base: Gpr, index: Gpr },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemoryAccess {
    pub access: Access,
    pub address: Address,
    pub size: i64,
}

impl MemoryAccess {
    /// Whether this access may touch `[offset, offset + size)` relative to
    /// the value `gpr` holds when the access executes.
    pub fn may_overlap(&self, gpr: Gpr, offset: i64, size: i64) -> bool {
        match self.address {
            Address::Displacement {
                base,
                offset: Some(start),
            } if base == gpr => start < offset + size && offset < start + self.size,
            Address::Displacement { base, offset: None } => base == gpr,
            Address::Indexed { base, index } => base == gpr || index == gpr,
            Address::Displacement { .. } => false,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LinkAccess {
    pub read: bool,
    pub write: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Effects {
    pub defs: GprSet,
    pub uses: GprSet,
    pub link: LinkAccess,
    pub memory: Option<MemoryAccess>,
}

impl Effects {
    pub fn touches(&self, gpr: Gpr) -> bool {
        self.defs.contains(gpr) || self.uses.contains(gpr)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Role {
    Def,
    Use,
    DefUse,
    /// Immediate, condition register field, or FPR.
    Other,
    /// `displacement(base)` used as an address without a memory access.
    Address,
    /// `displacement(base)` memory operand.
    Memory {
        access: Access,
        size: i64,
    },
    /// `displacement(base)` memory operand that writes the address back.
    MemoryUpdate {
        access: Access,
        size: i64,
    },
    /// Base register of an indexed memory access; the next operand is
    /// the index register.
    IndexedBase {
        access: Access,
        size: i64,
    },
    /// Indexed base register that receives the address.
    IndexedBaseUpdate {
        access: Access,
        size: i64,
    },
    IndexedIndex,
}

use Access::Load;
use Access::Store;
use Role::Def;
use Role::DefUse;
use Role::Other;
use Role::Use;

fn roles(mnemonic: &str, operand_count: usize) -> Option<Vec<Role>> {
    let mem = |access, size| Role::Memory { access, size };
    let update = |access, size| Role::MemoryUpdate { access, size };
    let indexed = |access, size| Role::IndexedBase { access, size };
    let indexed_update = |access, size| Role::IndexedBaseUpdate { access, size };
    let normalized = mnemonic.strip_suffix('.').unwrap_or(mnemonic);
    let roles = match normalized {
        "li" | "lis" => vec![Def, Other],
        "mr" | "not" | "neg" | "extsb" | "extsh" | "cntlzw" | "addze" | "addme" | "subfze"
        | "subfme" => vec![Def, Use],
        "addi" | "addis" | "addic" | "subfic" | "mulli" | "ori" | "oris" | "xori" | "xoris"
        | "andi" | "andis" | "srawi" | "slwi" | "srwi" | "clrlwi" | "clrrwi" | "rotlwi"
        | "rotrwi" => vec![Def, Use, Other],
        "extlwi" | "extrwi" | "clrlslwi" => vec![Def, Use, Other, Other],
        "rlwinm" => vec![Def, Use, Other, Other, Other],
        "rlwnm" => vec![Def, Use, Use, Other, Other],
        "rlwimi" => vec![DefUse, Use, Other, Other, Other],
        "insrwi" | "inslwi" => vec![DefUse, Use, Other, Other],
        "add" | "addc" | "adde" | "subf" | "subfc" | "subfe" | "sub" | "subc" | "and" | "andc"
        | "or" | "orc" | "xor" | "nor" | "nand" | "eqv" | "slw" | "srw" | "sraw" | "mullw"
        | "mulhw" | "mulhwu" | "divw" | "divwu" | "rotlw" => vec![Def, Use, Use],
        "la" => vec![Def, Role::Address],
        "isel" => vec![Def, Use, Use, Other],
        "mfcr" | "mfctr" => vec![Def],
        "mtctr" => vec![Use],
        "mflr" => vec![Def],
        "mtlr" => vec![Use],
        "mtcrf" | "mtocrf" => vec![Other, Use],
        "cmpwi" | "cmplwi" if operand_count == 2 => vec![Use, Other],
        "cmpwi" | "cmplwi" => vec![Other, Use, Other],
        "cmpw" | "cmplw" if operand_count == 2 => vec![Use, Use],
        "cmpw" | "cmplw" => vec![Other, Use, Use],
        "crmove" | "crxor" | "creqv" | "crand" | "crandc" | "cror" | "crnot" | "crnor"
        | "crnand" | "crorc" | "crset" | "crclr" | "mcrf" | "fmr" | "fadd" | "fadds" | "fsub"
        | "fsubs" | "fmul" | "fmuls" | "fdiv" | "fdivs" | "frsp" | "fctiwz" | "fctiw" | "fabs"
        | "fneg" | "fcmpu" | "fcmpo" | "fmadd" | "fmadds" | "fmsub" | "fmsubs" | "fnmadd"
        | "fnmsub" | "fsel" | "nop" => vec![Other; operand_count],
        "lwz" => vec![Def, mem(Load, 4)],
        "lbz" => vec![Def, mem(Load, 1)],
        "lhz" | "lha" => vec![Def, mem(Load, 2)],
        "lwzu" => vec![Def, update(Load, 4)],
        "lbzu" => vec![Def, update(Load, 1)],
        "lhzu" | "lhau" => vec![Def, update(Load, 2)],
        "lwzx" | "lwbrx" => vec![Def, indexed(Load, 4), Role::IndexedIndex],
        "lbzx" => vec![Def, indexed(Load, 1), Role::IndexedIndex],
        "lhzx" | "lhax" | "lhbrx" => vec![Def, indexed(Load, 2), Role::IndexedIndex],
        "lwzux" => vec![Def, indexed_update(Load, 4), Role::IndexedIndex],
        "lbzux" => vec![Def, indexed_update(Load, 1), Role::IndexedIndex],
        "lhzux" => vec![Def, indexed_update(Load, 2), Role::IndexedIndex],
        "lfs" => vec![Other, mem(Load, 4)],
        "lfd" => vec![Other, mem(Load, 8)],
        "lfsu" => vec![Other, update(Load, 4)],
        "lfdu" => vec![Other, update(Load, 8)],
        "lfsx" => vec![Other, indexed(Load, 4), Role::IndexedIndex],
        "lfdx" => vec![Other, indexed(Load, 8), Role::IndexedIndex],
        "stw" => vec![Use, mem(Store, 4)],
        "stb" => vec![Use, mem(Store, 1)],
        "sth" => vec![Use, mem(Store, 2)],
        "stwu" => vec![Use, update(Store, 4)],
        "stbu" => vec![Use, update(Store, 1)],
        "sthu" => vec![Use, update(Store, 2)],
        "stwx" | "stwbrx" => vec![Use, indexed(Store, 4), Role::IndexedIndex],
        "stbx" => vec![Use, indexed(Store, 1), Role::IndexedIndex],
        "sthx" | "sthbrx" => vec![Use, indexed(Store, 2), Role::IndexedIndex],
        "stwux" => vec![Use, indexed_update(Store, 4), Role::IndexedIndex],
        "stbux" => vec![Use, indexed_update(Store, 1), Role::IndexedIndex],
        "sthux" => vec![Use, indexed_update(Store, 2), Role::IndexedIndex],
        "stfs" => vec![Other, mem(Store, 4)],
        "stfd" => vec![Other, mem(Store, 8)],
        "stfsu" => vec![Other, update(Store, 4)],
        "stfdu" => vec![Other, update(Store, 8)],
        "stfsx" | "stfiwx" => vec![Other, indexed(Store, 4), Role::IndexedIndex],
        "stfdx" => vec![Other, indexed(Store, 8), Role::IndexedIndex],
        _ => return None,
    };
    (roles.len() == operand_count).then_some(roles)
}

/// GPR, link register, and memory effects; `None` for instructions without
/// a model and for control transfers.
pub fn effects(instruction: &Instruction<'_>) -> Option<Effects> {
    let roles = roles(instruction.mnemonic, instruction.operands.len())?;
    let mut effects = Effects {
        defs: GprSet::default(),
        uses: GprSet::default(),
        link: LinkAccess {
            read: instruction.mnemonic == "mflr",
            write: instruction.mnemonic == "mtlr",
        },
        memory: None,
    };
    let mut pending_index: Option<(Access, i64, Gpr)> = None;
    for (role, operand) in roles.iter().zip(&instruction.operands) {
        match (*role, *operand) {
            (Other, _) => {}
            (Def | Use | DefUse, Operand::Integer(number)) => {
                let gpr = Gpr::new(number)?;
                if matches!(role, Def | DefUse) {
                    effects.defs.insert(gpr);
                }
                if matches!(role, Use | DefUse) {
                    effects.uses.insert(gpr);
                }
            }
            (
                Role::Address | Role::Memory { .. } | Role::MemoryUpdate { .. },
                Operand::Memory { displacement, base },
            ) => {
                let base = Gpr::new(base)?;
                effects.uses.insert(base);
                let (access, size) = match *role {
                    Role::Memory { access, size } => (access, size),
                    Role::MemoryUpdate { access, size } => {
                        effects.defs.insert(base);
                        (access, size)
                    }
                    _ => continue,
                };
                let offset = match displacement {
                    Displacement::Integer(value) => Some(value),
                    Displacement::Expression(_) => None,
                };
                effects.memory = Some(MemoryAccess {
                    access,
                    address: Address::Displacement { base, offset },
                    size,
                });
            }
            (
                Role::IndexedBase { access, size } | Role::IndexedBaseUpdate { access, size },
                Operand::Integer(number),
            ) => {
                let base = Gpr::new(number)?;
                effects.uses.insert(base);
                if matches!(role, Role::IndexedBaseUpdate { .. }) {
                    effects.defs.insert(base);
                }
                pending_index = Some((access, size, base));
            }
            (Role::IndexedIndex, Operand::Integer(number)) => {
                let index = Gpr::new(number)?;
                effects.uses.insert(index);
                let (access, size, base) = pending_index.take()?;
                effects.memory = Some(MemoryAccess {
                    access,
                    address: Address::Indexed { base, index },
                    size,
                });
            }
            _ => return None,
        }
    }
    Some(effects)
}

/// Control transfer of an instruction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Control<'a> {
    Fallthrough,
    /// Direct branch; the target is a label or symbol expression.
    Jump {
        target: &'a str,
        conditional: bool,
    },
    /// Branch that writes the link register and returns to the next
    /// instruction.
    Call,
    /// Branch to the link register.
    Return {
        conditional: bool,
    },
    /// Branch to the count register.
    IndirectJump {
        conditional: bool,
    },
    /// Unconditional trap.
    Trap,
    Unknown,
}

/// Condition prefixes of extended branch mnemonics, longest first.
const BRANCH_CONDITIONS: [&str; 20] = [
    "dnzt", "dnzf", "dzt", "dzf", "dnz", "dz", "lt", "le", "eq", "ge", "gt", "nl", "ne", "ng",
    "so", "ns", "un", "nu", "t", "f",
];

pub fn control<'a>(instruction: &Instruction<'a>) -> Control<'a> {
    let mnemonic = instruction.mnemonic.trim_end_matches(['+', '-']);
    let target = || match instruction.operands.last() {
        Some(Operand::Expression(target)) => Some(*target),
        _ => None,
    };
    let jump = |conditional| match target() {
        Some(target) => Control::Jump {
            target,
            conditional,
        },
        None => Control::Unknown,
    };
    let Some(rest) = mnemonic.strip_prefix('b') else {
        return match mnemonic {
            "trap" => Control::Trap,
            "sc" | "rfi" | "rfid" | "hrfid" | "attn" => Control::Unknown,
            _ => Control::Fallthrough,
        };
    };
    match rest {
        "" | "a" => jump(false),
        "l" | "la" | "ctrl" => Control::Call,
        "lr" => Control::Return { conditional: false },
        "ctr" => Control::IndirectJump { conditional: false },
        "c" | "ca" => jump(true),
        "clr" => Control::Return { conditional: true },
        "cctr" => Control::IndirectJump { conditional: true },
        _ => {
            let Some(condition) = BRANCH_CONDITIONS.iter().find(|c| rest.starts_with(**c)) else {
                return Control::Unknown;
            };
            match &rest[condition.len()..] {
                "" | "a" => jump(true),
                "lr" => Control::Return { conditional: true },
                "ctr" => Control::IndirectJump { conditional: true },
                _ => Control::Unknown,
            }
        }
    }
}

/// Link register special purpose register number.
const SPR_LR: i64 = 8;

/// Link register access of a non-branch instruction; `None` when unknown.
pub fn link_access(instruction: &Instruction<'_>) -> Option<LinkAccess> {
    match instruction.mnemonic {
        "mflr" => Some(LinkAccess {
            read: true,
            write: false,
        }),
        "mtlr" => Some(LinkAccess {
            read: false,
            write: true,
        }),
        "mfspr" => match instruction.operands.as_slice() {
            [_, Operand::Integer(spr)] => Some(LinkAccess {
                read: *spr == SPR_LR,
                write: false,
            }),
            _ => None,
        },
        "mtspr" => match instruction.operands.as_slice() {
            [Operand::Integer(spr), _] => Some(LinkAccess {
                read: false,
                write: *spr == SPR_LR,
            }),
            _ => None,
        },
        _ => Some(LinkAccess::default()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line::Line;

    fn instruction(text: &str) -> Instruction<'_> {
        match Line::parse(text) {
            Line::Instruction(instruction) => instruction,
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn classifies_branches() {
        let cases = [
            (
                "\tb .LBB0_3",
                Control::Jump {
                    target: ".LBB0_3",
                    conditional: false,
                },
            ),
            (
                "\tbc 4, 1, .LBB0_2",
                Control::Jump {
                    target: ".LBB0_2",
                    conditional: true,
                },
            ),
            (
                "\tbgt-\t0, .LBB3_41",
                Control::Jump {
                    target: ".LBB3_41",
                    conditional: true,
                },
            ),
            (
                "\tbdnz .LBB1_1",
                Control::Jump {
                    target: ".LBB1_1",
                    conditional: true,
                },
            ),
            (
                "\tbf 1, .L_1",
                Control::Jump {
                    target: ".L_1",
                    conditional: true,
                },
            ),
            ("\tbl foo", Control::Call),
            ("\tbctrl", Control::Call),
            ("\tblr", Control::Return { conditional: false }),
            ("\tbeqlr\t0", Control::Return { conditional: true }),
            ("\tbltlr+\t0", Control::Return { conditional: true }),
            ("\tbdzlr", Control::Return { conditional: true }),
            ("\tbclr 12, 0, 0", Control::Return { conditional: true }),
            ("\tbctr", Control::IndirectJump { conditional: false }),
            ("\tbnectr 0", Control::IndirectJump { conditional: true }),
            ("\tbeql 0, foo", Control::Unknown),
            ("\tbcl 20, 31, .L1", Control::Unknown),
            ("\tblrl", Control::Unknown),
            ("\ttrap", Control::Trap),
            ("\tsc", Control::Unknown),
            ("\tmr 3, 4", Control::Fallthrough),
        ];
        for (text, expected) in cases {
            assert_eq!(control(&instruction(text)), expected, "{text}");
        }
    }

    #[test]
    fn models_effects() {
        let effects_of = |text| effects(&instruction(text)).expect(text);
        let store = effects_of("\tstw 30, 8(1)");
        assert_eq!(store.uses, [Gpr(30), Gpr::R1].into_iter().collect());
        assert!(store.defs.is_empty());
        assert_eq!(
            store.memory,
            Some(MemoryAccess {
                access: Access::Store,
                address: Address::Displacement {
                    base: Gpr::R1,
                    offset: Some(8)
                },
                size: 4
            })
        );
        let update = effects_of("\tstwu 1, -16(1)");
        assert!(update.defs.contains(Gpr::R1));
        let li = effects_of("\tli 3, 30");
        assert_eq!(li.defs, [Gpr(3)].into_iter().collect());
        assert!(li.uses.is_empty());
        let compare = effects_of("\tcmplwi\t3, 10035");
        assert_eq!(compare.uses, [Gpr(3)].into_iter().collect());
        let compare_field = effects_of("\tcmpw 1, 3, 4");
        assert_eq!(compare_field.uses, [Gpr(3), Gpr(4)].into_iter().collect());
        let insert = effects_of("\trlwimi 4, 4, 8, 16, 23");
        assert!(insert.uses.contains(Gpr(4)) && insert.defs.contains(Gpr(4)));
        let indexed = effects_of("\tlwzx 11, 10, 11");
        assert_eq!(
            indexed.memory.map(|m| m.address),
            Some(Address::Indexed {
                base: Gpr(10),
                index: Gpr(11)
            })
        );
        assert!(effects_of("\tmflr 0").link.read);
        assert!(effects_of("\tmtlr 0").link.write);
        assert_eq!(effects(&instruction("\tstwcx. 3, 0, 4")), None);
        assert_eq!(effects(&instruction("\tbl foo")), None);
        assert_eq!(effects(&instruction("\tli 3")), None);
    }
}
