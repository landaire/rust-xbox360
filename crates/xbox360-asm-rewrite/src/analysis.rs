//! Matching of callee-saved register save and restore sequences.
//!
//! Offsets are relative to the stack pointer at function entry (the
//! caller's stack pointer): GPR `rN` (14 <= N <= 31) is saved at
//! `-4 * (32 - N)` and the link register at `+4`, the SVR4 layout LLVM
//! uses when no FPRs are saved.
//!
//! Prologue: `mflr 0` and the frame allocation `stwu 1, -N(1)` are kept;
//! the stores of `rK..rM` (`M` is 30 or 31) and of `r0` to the link slot
//! become one `bl` before the allocation. The helper stores the same values
//! to the same addresses before the stack pointer moves.
//!
//! Epilogue: the loads of `rK..rM` and `r0`, `mtlr 0`, and `blr` become one
//! `b` after the deallocation `addi 1, 1, N`, which moves to the end of the
//! block. The helper loads the same addresses after the stack pointer moves.
//!
//! The helpers touch memory below the stack pointer between the helper call
//! and the allocation, and between the deallocation and the helper loads.
//! The Xbox 360 ABI keeps that area intact; the stock compiler's
//! `__savegprlr_N`/`__restgprlr_N` rely on the same guarantee.
//!
//! Every rule is a local equivalence argument over one straight-line block;
//! anything outside the effect model is refused. Save slots and the link
//! save word are fixed frame objects that are never address-taken, so only
//! accesses based on r1 can reach them. Conditions:
//!
//! - Prologue: the function has one `stwu 1, -N(1)`; `mflr 0`, the link
//!   store, and the saves are in its block. No instruction a store moves
//!   across writes the stored register or r1, or accesses its slot through
//!   r1; none between `mflr 0` and the call touches r0 or the link
//!   register. The lowest slot lies above the back chain word. On every
//!   path from the allocation, a call or `mtlr` writes the link register
//!   before a return, `mflr`, or branch out of the function reads it, and
//!   no path reaches inline assembly, an indirect branch, or the end of the
//!   function.
//! - Epilogue: in the block ending in `blr`, no instruction after a moved
//!   load uses or writes the loaded register, writes r1, or stores to the
//!   slot; none after `mtlr 0` accesses the link register; none after the
//!   deallocation uses r1.
//! - Functions with a relative branch other than `.+4`, or with several
//!   statements on one line, are left unchanged. Inline assembly never
//!   holds a candidate.

use std::collections::HashMap;

use crate::isa;
use crate::isa::Access;
use crate::isa::Control;
use crate::isa::Effects;
use crate::isa::Gpr;
use crate::isa::GprSet;
use crate::line::Displacement;
use crate::line::Instruction;
use crate::line::Line;
use crate::line::Operand;

mod error;

pub use self::error::LivePath;
pub use self::error::Refusal;

/// Offset of the link register save word from the incoming stack pointer.
const LINK_SLOT: i64 = 4;
const WORD: i64 = 4;

/// Size of a stack frame, as allocated by `stwu 1, -N(1)` and released by
/// `addi 1, 1, N`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameSize(i64);

impl FrameSize {
    pub fn bytes(self) -> i64 {
        self.0
    }
}

/// Registers `first..=last` saved or restored by one helper; `last` is r30
/// or r31 because LLVM allocates callee-saved GPRs from r30 down and r31
/// last.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SaveRange {
    pub first: Gpr,
    pub last: Gpr,
}

impl SaveRange {
    pub fn from_set(set: GprSet) -> Option<Self> {
        let first = set.first()?;
        let last = set.last()?;
        let contiguous = u32::from(last.number() - first.number()) + 1 == set.len();

        (first.is_callee_saved() && contiguous && (last == Gpr::R30 || last == Gpr::R31))
            .then_some(Self { first, last })
    }

    pub fn registers(self) -> impl Iterator<Item = Gpr> {
        (self.first.number()..=self.last.number()).filter_map(|number| Gpr::new(i64::from(number)))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrologueRewrite {
    /// Index of `stwu 1, -N(1)`; the helper call goes before it.
    pub allocation: usize,
    /// `mflr 0` that follows the allocation and moves before the call.
    pub moved_link_read: Option<usize>,
    /// Stores taken over by the helper, and `moved_link_read`.
    pub removed: Vec<usize>,
    pub range: SaveRange,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EpilogueRewrite {
    /// Index of `blr`, replaced by the deallocation and the helper branch.
    pub ret: usize,
    pub frame_size: FrameSize,
    /// Loads, `mtlr 0`, and the deallocation taken over or moved.
    pub removed: Vec<usize>,
    /// `None` restores only the link register.
    pub range: Option<SaveRange>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FunctionAnalysis {
    /// `None` when the function saves no callee-saved GPR after a frame
    /// allocation.
    pub prologue: Option<Result<PrologueRewrite, Refusal>>,
    /// One entry per `blr` whose block releases a frame.
    pub epilogues: Vec<Result<EpilogueRewrite, Refusal>>,
    pub inline_asm: bool,
}

/// Body of one compiled function, between `.Lfunc_beginN:` and
/// `.Lfunc_endN:`.
pub struct Function<'a> {
    lines: Vec<Line<'a>>,
    /// Lines between `#APP` and `#NO_APP`, inclusive.
    inline_asm: Vec<bool>,
}

impl<'a> Function<'a> {
    pub fn new(lines: Vec<Line<'a>>) -> Self {
        let mut inside = false;
        let inline_asm = lines
            .iter()
            .map(|line| match line {
                Line::InlineAsmStart => {
                    inside = true;
                    true
                }
                Line::InlineAsmEnd => {
                    inside = false;
                    true
                }
                _ => inside,
            })
            .collect();

        Self { lines, inline_asm }
    }

    fn instruction(&self, index: usize) -> Option<&Instruction<'a>> {
        match self.lines.get(index) {
            Some(Line::Instruction(instruction)) => Some(instruction),
            _ => None,
        }
    }

    /// Compiled instructions; inline assembly never holds a candidate.
    fn instructions(&self) -> impl Iterator<Item = (usize, &Instruction<'a>)> {
        self.lines
            .iter()
            .enumerate()
            .filter(|(index, _)| !self.inline_asm[*index])
            .filter_map(|(index, line)| match line {
                Line::Instruction(instruction) => Some((index, instruction)),
                _ => None,
            })
    }

    /// Control flow that block boundaries cannot follow: a relative branch
    /// may land inside a block, and joined statements hide their branches.
    fn unresolved_control(&self) -> Option<Refusal> {
        self.lines.iter().find_map(|line| match line {
            Line::Statements(_) => Some(Refusal::MultipleStatements),
            Line::Instruction(instruction) => match isa::control(instruction) {
                Control::Jump { target, .. }
                    if target != ".+4"
                        && (target == "."
                            || target.starts_with(".+")
                            || target.starts_with(".-")) =>
                {
                    Some(Refusal::RelativeBranch {
                        target: target.to_owned(),
                    })
                }
                _ => None,
            },
            _ => None,
        })
    }

    /// Straight-line block containing `index`: `[start, end)`. A block
    /// ends at labels, directives, inline assembly, and after control
    /// transfers.
    fn block(&self, index: usize) -> (usize, usize) {
        let straight = |line: &Line<'_>| match line {
            Line::Blank => true,
            Line::Instruction(instruction) => isa::control(instruction) == Control::Fallthrough,
            _ => false,
        };

        let mut start = index;
        while start > 0 && straight(&self.lines[start - 1]) {
            start -= 1;
        }

        let mut end = index + 1;
        while end < self.lines.len() && straight(&self.lines[end]) {
            end += 1;
        }

        (start, end)
    }

    pub fn analyze(&self) -> FunctionAnalysis {
        let inline_asm = self.lines.contains(&Line::InlineAsmStart);
        let prologue = self.analyze_prologue();
        let returns = self
            .instructions()
            .filter(|(_, instruction)| is_plain_return(instruction))
            .filter_map(|(index, _)| self.analyze_epilogue(index));

        if let Some(refusal) = self.unresolved_control() {
            return FunctionAnalysis {
                prologue: prologue.map(|_| Err(refusal.clone())),
                epilogues: returns.map(|_| Err(refusal.clone())).collect(),
                inline_asm,
            };
        }

        let prologue_window = match &prologue {
            Some(Ok(rewrite)) => {
                let end = rewrite
                    .removed
                    .iter()
                    .copied()
                    .max()
                    .unwrap_or(rewrite.allocation);
                Some((rewrite.allocation, end))
            }
            _ => None,
        };

        let epilogues = returns
            .map(|result| {
                let rewrite = result?;
                let start = rewrite.removed.iter().copied().min().unwrap_or(rewrite.ret);
                match prologue_window {
                    Some((low, high)) if start <= high && low <= rewrite.ret => {
                        Err(Refusal::OverlapsPrologue)
                    }
                    _ => Ok(rewrite),
                }
            })
            .collect();

        FunctionAnalysis {
            prologue,
            epilogues,
            inline_asm,
        }
    }

    fn analyze_prologue(&self) -> Option<Result<PrologueRewrite, Refusal>> {
        let allocations: Vec<(usize, FrameSize)> = self
            .instructions()
            .filter_map(|(index, instruction)| Some((index, allocation(instruction)?)))
            .collect();

        let (allocation_index, frame) = match allocations.as_slice() {
            [] => return None,
            [single] => *single,
            [..] => {
                let saves_any = self.instructions().any(|(_, instruction)| {
                    store_word(instruction).is_some_and(|(gpr, _)| gpr.is_callee_saved())
                });
                return saves_any.then_some(Err(Refusal::MultipleAllocations));
            }
        };

        let (start, end) = self.block(allocation_index);
        let mut saves: [Option<usize>; 32] = [None; 32];
        let mut link_store = None;

        for index in allocation_index + 1..end {
            let Some((gpr, offset)) = self.instruction(index).and_then(store_word) else {
                continue;
            };

            let slot = offset - frame.0;
            if gpr == Gpr::R0 && slot == LINK_SLOT {
                link_store.get_or_insert(index);
            } else if gpr.is_callee_saved() && slot == save_slot(gpr) {
                saves[usize::from(gpr.number())].get_or_insert(index);
            }
        }

        let saved: GprSet = saves
            .iter()
            .enumerate()
            .filter(|(_, position)| position.is_some())
            .filter_map(|(number, _)| Gpr::new(number as i64))
            .collect();
        if saved.is_empty() {
            return None;
        }

        Some(self.plan_prologue(PrologueCandidate {
            allocation: allocation_index,
            frame,
            block_start: start,
            saves,
            saved,
            link_store,
        }))
    }

    fn plan_prologue(&self, candidate: PrologueCandidate) -> Result<PrologueRewrite, Refusal> {
        let PrologueCandidate {
            allocation: allocation_index,
            frame,
            block_start,
            saves,
            saved,
            link_store,
        } = candidate;

        let range =
            SaveRange::from_set(saved).ok_or(Refusal::NonContiguous { registers: saved })?;
        // `stwu` writes the back chain at [0, 4) of the new frame after the
        // helper stores.
        if frame.0 + save_slot(range.first) < WORD {
            return Err(Refusal::SlotOverlapsBackChain);
        }

        let link_store = link_store.ok_or(Refusal::LinkRegisterNotSaved)?;
        let link_read = (block_start..link_store)
            .rev()
            .find(|index| self.instruction(*index).is_some_and(is_link_read_to_r0))
            .ok_or(Refusal::LinkRegisterNotSaved)?;

        let mut stores: Vec<MovedStore> = range
            .registers()
            .filter_map(|gpr| {
                let position = saves[usize::from(gpr.number())]?;
                Some(MovedStore {
                    gpr,
                    position,
                    slot: frame.0 + save_slot(gpr),
                })
            })
            .collect();
        stores.push(MovedStore {
            gpr: Gpr::R0,
            position: link_store,
            slot: frame.0 + LINK_SLOT,
        });

        let moved_link_read = (link_read > allocation_index).then_some(link_read);
        let mut removed: Vec<usize> = stores.iter().map(|store| store.position).collect();
        removed.extend(moved_link_read);
        removed.sort_unstable();
        let window_end = removed.last().copied().unwrap_or(allocation_index);

        // r0 must still hold the `mflr 0` result at the helper call.
        for index in link_read + 1..allocation_index {
            let effects = self.effects_at(index)?;
            if let Some(effects) = effects {
                if effects.defs.contains(Gpr::R0) {
                    return Err(Refusal::WrittenBeforeSave { register: Gpr::R0 });
                }

                if effects.link.write {
                    return Err(Refusal::LinkRegisterAccessed);
                }
            }
        }

        for index in allocation_index + 1..window_end {
            if removed.binary_search(&index).is_ok() {
                continue;
            }

            let Some(effects) = self.effects_at(index)? else {
                continue;
            };

            if effects.defs.contains(Gpr::R1) {
                return Err(Refusal::StackPointerAccessed);
            }

            if let Some(link_read) = moved_link_read
                && index < link_read
                && (effects.touches(Gpr::R0) || effects.link.read || effects.link.write)
            {
                return Err(Refusal::LinkRegisterAccessed);
            }

            for store in &stores {
                if index > store.position {
                    continue;
                }

                if effects.defs.contains(store.gpr) {
                    return Err(Refusal::WrittenBeforeSave {
                        register: store.gpr,
                    });
                }

                if effects
                    .memory
                    .is_some_and(|memory| memory.may_overlap(Gpr::R1, store.slot, WORD))
                {
                    return Err(Refusal::SlotAccessed);
                }
            }
        }

        self.link_register_dead(allocation_index, &removed)
            .map_err(|path| Refusal::LinkRegisterLive { path })?;

        Ok(PrologueRewrite {
            allocation: allocation_index,
            moved_link_read,
            removed,
            range,
        })
    }

    /// `Ok(None)` for blank lines; instructions without a model refuse.
    fn effects_at(&self, index: usize) -> Result<Option<Effects>, Refusal> {
        match &self.lines[index] {
            Line::Instruction(instruction) => {
                isa::effects(instruction)
                    .map(Some)
                    .ok_or_else(|| Refusal::UnknownInstruction {
                        mnemonic: instruction.mnemonic.to_owned(),
                    })
            }
            _ => Ok(None),
        }
    }

    /// Whether every path from the allocation writes the link register
    /// before reading it; `skipped` instructions move before the helper
    /// call.
    fn link_register_dead(
        &self,
        allocation_index: usize,
        skipped: &[usize],
    ) -> Result<(), LivePath> {
        let labels: HashMap<&str, usize> = self
            .lines
            .iter()
            .enumerate()
            .filter_map(|(index, line)| match line {
                Line::Label(name) => Some((*name, index)),
                _ => None,
            })
            .collect();

        let mut visited = vec![false; self.lines.len()];
        let mut pending = vec![allocation_index + 1];

        while let Some(mut index) = pending.pop() {
            loop {
                let Some(line) = self.lines.get(index) else {
                    return Err(LivePath::FallsOffEnd);
                };

                if std::mem::replace(&mut visited[index], true) {
                    break;
                }

                match line {
                    // No effect on the link register.
                    Line::Label(_) | Line::Blank => {}
                    Line::Directive(directive) => {
                        if !is_alignment(directive) {
                            return Err(LivePath::Directive {
                                directive: (*directive).to_owned(),
                            });
                        }
                    }
                    Line::InlineAsmStart | Line::InlineAsmEnd => return Err(LivePath::InlineAsm),
                    Line::Statements(statements) => {
                        return Err(LivePath::UnknownControl {
                            mnemonic: (*statements).to_owned(),
                        });
                    }
                    // Moves before the helper call, so it does not run on this path.
                    Line::Instruction(_) if skipped.binary_search(&index).is_ok() => {}
                    Line::Instruction(instruction) => {
                        let mnemonic = || instruction.mnemonic.to_owned();
                        match isa::control(instruction) {
                            Control::Fallthrough => {
                                let access = isa::link_access(instruction).ok_or_else(|| {
                                    LivePath::UnknownAccess {
                                        mnemonic: mnemonic(),
                                    }
                                })?;

                                if access.read {
                                    return Err(LivePath::Read {
                                        mnemonic: mnemonic(),
                                    });
                                }
                                if access.write {
                                    break;
                                }
                            }
                            Control::Call | Control::Trap => break,
                            Control::Return { .. } => {
                                return Err(LivePath::Read {
                                    mnemonic: mnemonic(),
                                });
                            }
                            Control::IndirectJump { .. } => return Err(LivePath::IndirectBranch),
                            Control::Unknown => {
                                return Err(LivePath::UnknownControl {
                                    mnemonic: mnemonic(),
                                });
                            }
                            // `.+4` targets the next instruction (LLVM's
                            // acquire idiom `bne- 7, .+4; isync`).
                            Control::Jump { target: ".+4", .. } => {}
                            Control::Jump {
                                target,
                                conditional,
                            } => {
                                let Some(&destination) = labels.get(target) else {
                                    return Err(LivePath::TailBranch {
                                        target: target.to_owned(),
                                    });
                                };

                                if conditional {
                                    pending.push(index + 1);
                                }
                                index = destination;
                                continue;
                            }
                        }
                    }
                }

                index += 1;
            }
        }

        Ok(())
    }

    fn analyze_epilogue(&self, ret: usize) -> Option<Result<EpilogueRewrite, Refusal>> {
        let (start, _) = self.block(ret);
        let (deallocation, frame) = (start..ret)
            .rev()
            .find_map(|index| Some((index, deallocation(self.instruction(index)?)?)))?;

        // Offset from the incoming stack pointer of `offset(1)` at `index`.
        let slot_of = |index: usize, offset: i64| {
            if index < deallocation {
                offset - frame.0
            } else {
                offset
            }
        };

        let mut restores: [Option<usize>; 32] = [None; 32];
        let mut link_loads = Vec::new();

        for index in start..ret {
            let Some((gpr, offset)) = self.instruction(index).and_then(load_word) else {
                continue;
            };
            let slot = slot_of(index, offset);
            if gpr == Gpr::R0 && slot == LINK_SLOT {
                link_loads.push(index);
            } else if gpr.is_callee_saved() && slot == save_slot(gpr) {
                restores[usize::from(gpr.number())] = Some(index);
            }
        }

        let restored: GprSet = restores
            .iter()
            .enumerate()
            .filter(|(_, position)| position.is_some())
            .filter_map(|(number, _)| Gpr::new(number as i64))
            .collect();
        let link_write = (start..ret)
            .rev()
            .find(|index| self.instruction(*index).is_some_and(is_link_write_from_r0));
        let Some(link_write) = link_write else {
            return (!restored.is_empty()).then_some(Err(Refusal::LinkRegisterNotRestored));
        };

        Some(self.plan_epilogue(EpilogueCandidate {
            ret,
            deallocation,
            frame,
            restores,
            restored,
            link_loads,
            link_write,
        }))
    }

    fn plan_epilogue(&self, candidate: EpilogueCandidate) -> Result<EpilogueRewrite, Refusal> {
        let EpilogueCandidate {
            ret,
            deallocation,
            frame,
            restores,
            restored,
            link_loads,
            link_write,
        } = candidate;

        let link_load = link_loads
            .iter()
            .copied()
            .rfind(|index| *index < link_write)
            .ok_or(Refusal::LinkRegisterNotRestored)?;

        let range = if restored.is_empty() {
            None
        } else {
            Some(SaveRange::from_set(restored).ok_or(Refusal::NonContiguous {
                registers: restored,
            })?)
        };

        #[derive(Clone, Copy)]
        enum Moved {
            Restore { gpr: Gpr, slot: i64 },
            Deallocation,
            LinkWrite,
        }

        let frame_offset = |index: usize, slot: i64| {
            // Offset relative to r1 as it is at `index`.
            if index < deallocation {
                slot + frame.0
            } else {
                slot
            }
        };

        let mut moved: Vec<(usize, Moved)> = range
            .iter()
            .flat_map(|range| range.registers())
            .filter_map(|gpr| {
                let position = restores[usize::from(gpr.number())]?;
                Some((
                    position,
                    Moved::Restore {
                        gpr,
                        slot: save_slot(gpr),
                    },
                ))
            })
            .collect();
        moved.push((
            link_load,
            Moved::Restore {
                gpr: Gpr::R0,
                slot: LINK_SLOT,
            },
        ));
        moved.push((link_write, Moved::LinkWrite));
        moved.push((deallocation, Moved::Deallocation));
        moved.sort_unstable_by_key(|(position, _)| *position);

        let removed: Vec<usize> = moved.iter().map(|(position, _)| *position).collect();
        let first = removed[0];

        for index in first + 1..ret {
            if removed.binary_search(&index).is_ok() {
                continue;
            }

            let Some(effects) = self.effects_at(index)? else {
                continue;
            };

            for (position, kind) in &moved {
                if *position > index {
                    break;
                }

                match *kind {
                    Moved::Restore { gpr, slot } => {
                        if effects.touches(gpr) {
                            return Err(Refusal::AccessedAfterRestore { register: gpr });
                        }

                        if effects.defs.contains(Gpr::R1) {
                            return Err(Refusal::StackPointerAccessed);
                        }

                        if effects.memory.is_some_and(|memory| {
                            memory.access == Access::Store
                                && memory.may_overlap(Gpr::R1, frame_offset(index, slot), WORD)
                        }) {
                            return Err(Refusal::SlotAccessed);
                        }
                    }
                    Moved::LinkWrite => {
                        if effects.link.read || effects.link.write {
                            return Err(Refusal::LinkRegisterAccessed);
                        }
                    }
                    Moved::Deallocation => {
                        if effects.touches(Gpr::R1) {
                            return Err(Refusal::StackPointerAccessed);
                        }
                    }
                }
            }
        }

        Ok(EpilogueRewrite {
            ret,
            frame_size: frame,
            removed,
            range,
        })
    }
}

/// Saves found after a frame allocation.
struct PrologueCandidate {
    allocation: usize,
    frame: FrameSize,
    block_start: usize,
    /// Position of the first store of each register to its slot.
    saves: [Option<usize>; 32],
    saved: GprSet,
    link_store: Option<usize>,
}

/// Restores found in a block that ends in `blr`.
struct EpilogueCandidate {
    ret: usize,
    deallocation: usize,
    frame: FrameSize,
    /// Position of the last load of each register from its slot.
    restores: [Option<usize>; 32],
    restored: GprSet,
    link_loads: Vec<usize>,
    link_write: usize,
}

/// Store taken over by the save helper, with its offset in the new frame.
struct MovedStore {
    gpr: Gpr,
    position: usize,
    slot: i64,
}

/// Offset of the save slot of `gpr` from the incoming stack pointer.
pub fn save_slot(gpr: Gpr) -> i64 {
    -WORD * (32 - i64::from(gpr.number()))
}

fn is_alignment(directive: &str) -> bool {
    let name = directive.split_whitespace().next().unwrap_or(directive);
    matches!(
        name,
        ".p2align" | ".p2alignl" | ".p2alignw" | ".balign" | ".align"
    )
}

fn register(operand: &Operand<'_>) -> Option<Gpr> {
    match operand {
        Operand::Integer(number) => Gpr::new(*number),
        _ => None,
    }
}

fn stack_operand(operand: &Operand<'_>) -> Option<i64> {
    match operand {
        Operand::Memory {
            displacement: Displacement::Integer(offset),
            base: 1,
        } => Some(*offset),
        _ => None,
    }
}

/// `stw rS, offset(1)`.
fn store_word(instruction: &Instruction<'_>) -> Option<(Gpr, i64)> {
    match (instruction.mnemonic, instruction.operands.as_slice()) {
        ("stw", [source, address]) => Some((register(source)?, stack_operand(address)?)),
        _ => None,
    }
}

/// `lwz rD, offset(1)`.
fn load_word(instruction: &Instruction<'_>) -> Option<(Gpr, i64)> {
    match (instruction.mnemonic, instruction.operands.as_slice()) {
        ("lwz", [destination, address]) => Some((register(destination)?, stack_operand(address)?)),
        _ => None,
    }
}

/// `stwu 1, -N(1)`.
fn allocation(instruction: &Instruction<'_>) -> Option<FrameSize> {
    match (instruction.mnemonic, instruction.operands.as_slice()) {
        ("stwu", [Operand::Integer(1), address]) => {
            let offset = stack_operand(address)?;
            (offset < 0).then_some(FrameSize(-offset))
        }
        _ => None,
    }
}

/// `addi 1, 1, N`.
fn deallocation(instruction: &Instruction<'_>) -> Option<FrameSize> {
    match (instruction.mnemonic, instruction.operands.as_slice()) {
        (
            "addi",
            [
                Operand::Integer(1),
                Operand::Integer(1),
                Operand::Integer(size),
            ],
        ) if *size > 0 => Some(FrameSize(*size)),
        _ => None,
    }
}

fn is_link_read_to_r0(instruction: &Instruction<'_>) -> bool {
    instruction.mnemonic == "mflr" && instruction.operands == [Operand::Integer(0)]
}

fn is_link_write_from_r0(instruction: &Instruction<'_>) -> bool {
    instruction.mnemonic == "mtlr" && instruction.operands == [Operand::Integer(0)]
}

fn is_plain_return(instruction: &Instruction<'_>) -> bool {
    instruction.mnemonic == "blr" && instruction.operands.is_empty()
}
