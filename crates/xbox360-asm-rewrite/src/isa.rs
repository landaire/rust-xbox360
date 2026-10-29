//! Register, memory, link register, and control effects of PowerPC
//! instructions as printed by LLVM.
//!
//! The GPR effect model covers the instructions that LLVM schedules into
//! prologues and epilogues. Instructions outside it have no model, which
//! every rule treats as a reason to refuse.
//!
//! The model also covers the AltiVec instructions and those of the Xenon
//! VMX128 extension. LLVM prints vector registers (v0-v127) as bare
//! numbers, like GPRs; the mnemonic decides, so a vector register operand
//! never counts as a GPR. Vector loads and stores (`lvx128 vD, rA, rB`) are
//! indexed accesses through a GPR base and index, which may touch any
//! address relative to either register: that covers the aligned 16 bytes
//! that `lvx`/`stvx` forms access, the part of them that `lvlx`/`lvrx`/
//! `stvlx`/`stvrx` forms access, and the element of `lvewx`/`stvewx` forms.
//! `lvsl`/`lvsr` forms read their address registers without accessing
//! memory. All other vector instructions only read and write vector
//! registers, VSCR, and, in record form, condition register field 6, none
//! of which a rule tracks. A base register `0`, which stands for the value
//! zero, counts as a use of r0, as in the integer indexed forms.

use std::fmt;

use crate::line::Displacement;
use crate::line::Instruction;
use crate::line::Operand;

use self::Access::Load;
use self::Access::Store;
use self::Role::Def;
use self::Role::DefUse;
use self::Role::Other;
use self::Role::Use;

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

    pub fn new(number: i64) -> Option<Self> {
        u8::try_from(number)
            .ok()
            .filter(|number| *number < 32)
            .map(Self)
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
        (0..32u8)
            .filter(move |number| self.0 & (1 << number) != 0)
            .map(Gpr)
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
        let mut set = Self::default();

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
    /// `base + index`; vector forms align the sum down to 16 bytes.
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Role {
    Def,
    Use,
    DefUse,
    /// Immediate, condition register field, FPR, or vector register.
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

/// Indexed base register waiting for the index operand.
struct PendingIndex {
    access: Access,
    size: i64,
    base: Gpr,
}

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
        "lvx" | "lvxl" | "lvlx" | "lvlxl" | "lvrx" | "lvrxl" | "lvx128" | "lvxl128" | "lvlx128"
        | "lvlxl128" | "lvrx128" | "lvrxl128" => {
            vec![Other, indexed(Load, 16), Role::IndexedIndex]
        }
        "lvebx" => vec![Other, indexed(Load, 1), Role::IndexedIndex],
        "lvehx" => vec![Other, indexed(Load, 2), Role::IndexedIndex],
        "lvewx" | "lvewx128" => vec![Other, indexed(Load, 4), Role::IndexedIndex],
        "stvx" | "stvxl" | "stvlx" | "stvlxl" | "stvrx" | "stvrxl" | "stvx128" | "stvxl128"
        | "stvlx128" | "stvlxl128" | "stvrx128" | "stvrxl128" => {
            vec![Other, indexed(Store, 16), Role::IndexedIndex]
        }
        "stvebx" => vec![Other, indexed(Store, 1), Role::IndexedIndex],
        "stvehx" => vec![Other, indexed(Store, 2), Role::IndexedIndex],
        "stvewx" | "stvewx128" => vec![Other, indexed(Store, 4), Role::IndexedIndex],
        // Permute control from the address; no memory access.
        "lvsl" | "lvsr" | "lvsl128" | "lvsr128" => vec![Other, Use, Use],
        // AltiVec operations on vector registers, immediates, and VSCR.
        "mfvscr" | "mtvscr" | "vaddcuw" | "vaddfp" | "vaddsbs" | "vaddshs" | "vaddsws"
        | "vaddubm" | "vaddubs" | "vadduhm" | "vadduhs" | "vadduwm" | "vadduws" | "vand"
        | "vandc" | "vavgsb" | "vavgsh" | "vavgsw" | "vavgub" | "vavguh" | "vavguw" | "vcfsx"
        | "vcfux" | "vcmpbfp" | "vcmpeqfp" | "vcmpequb" | "vcmpequh" | "vcmpequw" | "vcmpgefp"
        | "vcmpgtfp" | "vcmpgtsb" | "vcmpgtsh" | "vcmpgtsw" | "vcmpgtub" | "vcmpgtuh"
        | "vcmpgtuw" | "vctsxs" | "vctuxs" | "vexptefp" | "vlogefp" | "vmaddfp" | "vmaxfp"
        | "vmaxsb" | "vmaxsh" | "vmaxsw" | "vmaxub" | "vmaxuh" | "vmaxuw" | "vmhaddshs"
        | "vmhraddshs" | "vminfp" | "vminsb" | "vminsh" | "vminsw" | "vminub" | "vminuh"
        | "vminuw" | "vmladduhm" | "vmr" | "vmrghb" | "vmrghh" | "vmrghw" | "vmrglb" | "vmrglh"
        | "vmrglw" | "vmsummbm" | "vmsumshm" | "vmsumshs" | "vmsumubm" | "vmsumuhm"
        | "vmsumuhs" | "vmulesb" | "vmulesh" | "vmuleub" | "vmuleuh" | "vmulosb" | "vmulosh"
        | "vmuloub" | "vmulouh" | "vnmsubfp" | "vnor" | "vnot" | "vor" | "vperm" | "vpkpx"
        | "vpkshss" | "vpkshus" | "vpkswss" | "vpkswus" | "vpkuhum" | "vpkuhus" | "vpkuwum"
        | "vpkuwus" | "vrefp" | "vrfim" | "vrfin" | "vrfip" | "vrfiz" | "vrlb" | "vrlh"
        | "vrlw" | "vrsqrtefp" | "vsel" | "vsl" | "vslb" | "vsldoi" | "vslh" | "vslo" | "vslw"
        | "vspltb" | "vsplth" | "vspltisb" | "vspltish" | "vspltisw" | "vspltw" | "vsr"
        | "vsrab" | "vsrah" | "vsraw" | "vsrb" | "vsrh" | "vsro" | "vsrw" | "vsubcuw"
        | "vsubfp" | "vsubsbs" | "vsubshs" | "vsubsws" | "vsububm" | "vsububs" | "vsubuhm"
        | "vsubuhs" | "vsubuwm" | "vsubuws" | "vsum2sws" | "vsum4sbs" | "vsum4shs" | "vsum4ubs"
        | "vsumsws" | "vupkhpx" | "vupkhsb" | "vupkhsh" | "vupklpx" | "vupklsb" | "vupklsh"
        | "vxor" => vec![Other; operand_count],
        // VMX128 operations on vector registers and immediates.
        "vaddfp128" | "vand128" | "vandc128" | "vcfpsxws128" | "vcfpuxws128" | "vcmpbfp128"
        | "vcmpeqfp128" | "vcmpequw128" | "vcmpgefp128" | "vcmpgtfp128" | "vcsxwfp128"
        | "vcuxwfp128" | "vexptefp128" | "vlogefp128" | "vmaddcfp128" | "vmaddfp128"
        | "vmaxfp128" | "vminfp128" | "vmrghw128" | "vmrglw128" | "vmsum3fp128" | "vmsum4fp128"
        | "vmulfp128" | "vnmsubfp128" | "vnor128" | "vor128" | "vperm128" | "vpermwi128"
        | "vpkd3d128" | "vpkshss128" | "vpkshus128" | "vpkswss128" | "vpkswus128"
        | "vpkuhum128" | "vpkuhus128" | "vpkuwum128" | "vpkuwus128" | "vrefp128" | "vrfim128"
        | "vrfin128" | "vrfip128" | "vrfiz128" | "vrlimi128" | "vrlw128" | "vrsqrtefp128"
        | "vsel128" | "vsldoi128" | "vslo128" | "vslw128" | "vspltisw128" | "vspltw128"
        | "vsraw128" | "vsro128" | "vsrw128" | "vsubfp128" | "vupkd3d128" | "vupkhsb128"
        | "vupkhsh128" | "vupklsb128" | "vupklsh128" | "vxor128" => vec![Other; operand_count],
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

    let mut pending_index: Option<PendingIndex> = None;

    for (role, operand) in roles.iter().zip(&instruction.operands) {
        match (*role, *operand) {
            // Immediates, condition fields, and FPRs have no GPR effect.
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

                pending_index = Some(PendingIndex { access, size, base });
            }
            (Role::IndexedIndex, Operand::Integer(number)) => {
                let index = Gpr::new(number)?;
                effects.uses.insert(index);

                let pending = pending_index.take()?;
                effects.memory = Some(MemoryAccess {
                    access: pending.access,
                    address: Address::Indexed {
                        base: pending.base,
                        index,
                    },
                    size: pending.size,
                });
            }
            _ => return None,
        }
    }

    Some(effects)
}

pub fn control<'a>(instruction: &Instruction<'a>) -> Control<'a> {
    /// Condition prefixes of extended branch mnemonics, longest first.
    const BRANCH_CONDITIONS: [&str; 20] = [
        "dnzt", "dnzf", "dzt", "dzf", "dnz", "dz", "lt", "le", "eq", "ge", "gt", "nl", "ne", "ng",
        "so", "ns", "un", "nu", "t", "f",
    ];

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
            let Some(condition) = BRANCH_CONDITIONS
                .iter()
                .find(|prefix| rest.starts_with(**prefix))
            else {
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

/// Link register access of a non-branch instruction; `None` when unknown.
pub fn link_access(instruction: &Instruction<'_>) -> Option<LinkAccess> {
    /// Link register special purpose register number.
    const SPR_LR: i64 = 8;

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

    #[test]
    fn models_vector_effects() {
        let effects_of = |text| effects(&instruction(text)).expect(text);
        let save = effects_of("\tstvx128 127, 1, 12");
        assert_eq!(save.uses, [Gpr::R1, Gpr(12)].into_iter().collect());
        assert!(save.defs.is_empty());
        let save_access = MemoryAccess {
            access: Access::Store,
            address: Address::Indexed {
                base: Gpr::R1,
                index: Gpr(12),
            },
            size: 16,
        };
        assert_eq!(save.memory, Some(save_access));
        // The index register's value is unknown, so any slot may be hit.
        assert!(save_access.may_overlap(Gpr::R1, -8, 4));
        // A base of 0 counts as r0; v64 is no GPR.
        let load = effects_of("\tlvx128 64, 0, 3");
        assert_eq!(load.uses, [Gpr::R0, Gpr(3)].into_iter().collect());
        assert!(load.defs.is_empty());
        assert_eq!(
            load.memory.map(|m| (m.access, m.address)),
            Some((
                Access::Load,
                Address::Indexed {
                    base: Gpr::R0,
                    index: Gpr(3)
                }
            ))
        );
        assert!(!load.memory.expect("load").may_overlap(Gpr::R1, -8, 4));
        let partial = effects_of("\tstvrx128 5, 4, 1");
        assert_eq!(
            partial.memory.map(|m| (m.access, m.size)),
            Some((Access::Store, 16))
        );
        let element = effects_of("\tlvewx 2, 1, 12");
        assert_eq!(
            element.memory.map(|m| (m.access, m.size)),
            Some((Access::Load, 4))
        );
        let shift = effects_of("\tlvsl 2, 0, 3");
        assert_eq!(shift.uses, [Gpr::R0, Gpr(3)].into_iter().collect());
        assert_eq!(shift.memory, None);

        for text in [
            "\tvaddfp128 1, 2, 3",
            "\tvcmpeqfp128. 100, 2, 127",
            "\tvcmpequw. 0, 1, 2",
            "\tvrlimi128 64, 65, 8, 0",
            "\tvpkd3d128 1, 2, 3, 1, 3",
            "\tvspltisw128 99, -1",
            "\tvmr 2, 3",
            "\tmfvscr 3",
        ] {
            let operation = effects_of(text);
            assert!(operation.defs.is_empty(), "{text}");
            assert!(operation.uses.is_empty(), "{text}");
            assert_eq!(operation.memory, None, "{text}");
            assert_eq!(operation.link, LinkAccess::default(), "{text}");
        }

        // Data stream touches, and POWER instructions whose encodings
        // VMX128 reuses, stay outside the model.
        assert_eq!(effects(&instruction("\tdst 3, 4, 0")), None);
        assert_eq!(effects(&instruction("\tvinsd 31, 12, 1")), None);
        assert_eq!(effects(&instruction("\tlxvp 32, 6208(2)")), None);
    }
}
