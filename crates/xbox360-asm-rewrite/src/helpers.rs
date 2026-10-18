//! Shared save and restore routines.
//!
//! Each routine family is one fall-through chain, entered at the first
//! register of the range:
//!
//! - `__xbox360_savegprlr_K_M`, called with `bl` before `stwu 1, -N(1)`
//!   with the link register in r0: stores `rK..rM` at `-4 * (32 - n)(1)`
//!   and r0 at `4(1)`. Clobbers LR.
//! - `__xbox360_restgprlr_K_M`, entered with `b` after `addi 1, 1, N`:
//!   loads `rK..rM` from `-4 * (32 - n)(1)` and LR from `4(1)` through r0,
//!   then returns to the caller's caller.
//! - `__xbox360_restlr`: the link register part of the restore chain
//!   ending at r30.
//!
//! With `unwind_info`, each chain has call frame information. A save
//! routine runs before the caller's allocation, so its frame is the
//! caller's frame with the return address in LR, the initial state. It
//! keeps r0, which holds the caller's return address (see
//! [`crate::rewrite`]). A restore routine runs after the caller's
//! deallocation in place of the caller's return: the CFA is r1, the return
//! address is in its slot, and each register of the chain is in its slot
//! until the routine loads it. An entry inside a chain skips only loads of
//! registers that the caller did not save, which the caller left
//! unchanged.

use std::io;
use std::io::Write;

use crate::analysis::LINK_SLOT;
use crate::analysis::SaveRange;
use crate::analysis::save_slot;
use crate::isa::Gpr;

/// Chains ending at r30 and r31.
const CHAIN_ENDS: [Gpr; 2] = [Gpr::R30, Gpr::R31];

/// Lowest entry point of each chain that rewritten code references.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HelperUse {
    save_from: [Option<Gpr>; 2],
    restore_from: [Option<Gpr>; 2],
    restore_link_only: bool,
}

impl HelperUse {
    pub fn record_save(&mut self, range: SaveRange) {
        let entry = &mut self.save_from[chain_index(range.last)];
        *entry = Some(entry.map_or(range.first, |first| first.min(range.first)));
    }

    pub fn record_restore(&mut self, range: Option<SaveRange>) {
        match range {
            Some(range) => {
                let entry = &mut self.restore_from[chain_index(range.last)];
                *entry = Some(entry.map_or(range.first, |first| first.min(range.first)));
            }
            None => self.restore_link_only = true,
        }
    }

    /// Size of the emitted routines in instructions.
    pub fn instruction_count(&self) -> usize {
        let chain = |from: Option<Gpr>, end: Gpr, tail: usize| {
            from.map_or(0, |first| {
                usize::from(end.number() - first.number()) + 1 + tail
            })
        };

        let saves: usize = CHAIN_ENDS
            .iter()
            .zip(self.save_from)
            .map(|(end, from)| chain(from, *end, 2))
            .sum();
        let restore_r30 = match self.restore_from[0] {
            Some(_) => chain(self.restore_from[0], Gpr::R30, 3),
            None if self.restore_link_only => 3,
            None => 0,
        };

        saves + restore_r30 + chain(self.restore_from[1], Gpr::R31, 3)
    }

    pub fn emit(&self, out: &mut impl Write, unwind_info: bool) -> io::Result<()> {
        if *self == Self::default() {
            return Ok(());
        }

        let frame = Annotations { unwind_info };
        writeln!(out)?;
        writeln!(out, "\t.section\t.text.__xbox360_gprlr,\"ax\",@progbits")?;
        writeln!(out, "\t.p2align\t2")?;

        for (end, from) in CHAIN_ENDS.iter().zip(self.save_from) {
            let Some(first) = from else { continue };

            frame.line(out, ".cfi_startproc")?;
            frame.line(out, ".cfi_same_value r0")?;
            for gpr in registers(first, *end) {
                entry(
                    out,
                    &save_name(SaveRange {
                        first: gpr,
                        last: *end,
                    }),
                )?;
                writeln!(out, "\tstw {}, {}(1)", gpr.number(), save_slot(gpr))?;
            }

            writeln!(out, "\tstw 0, {LINK_SLOT}(1)")?;
            writeln!(out, "\tblr")?;
            frame.line(out, ".cfi_endproc")?;
        }

        let r30_chain = self.restore_from[0];
        if r30_chain.is_some() || self.restore_link_only {
            frame.restore_start(out, r30_chain.map(|first| (first, Gpr::R30)))?;

            if let Some(first) = r30_chain {
                for gpr in registers(first, Gpr::R30) {
                    entry(
                        out,
                        &restore_name(Some(SaveRange {
                            first: gpr,
                            last: Gpr::R30,
                        })),
                    )?;
                    restore(out, &frame, gpr)?;
                }
            }

            entry(out, &restore_name(None))?;
            restore_link(out)?;
            frame.line(out, ".cfi_endproc")?;
        }

        if let Some(first) = self.restore_from[1] {
            frame.restore_start(out, Some((first, Gpr::R31)))?;

            for gpr in registers(first, Gpr::R31) {
                entry(
                    out,
                    &restore_name(Some(SaveRange {
                        first: gpr,
                        last: Gpr::R31,
                    })),
                )?;
                restore(out, &frame, gpr)?;
            }

            restore_link(out)?;
            frame.line(out, ".cfi_endproc")?;
        }

        Ok(())
    }
}

/// Call frame annotations, written only with unwind information.
struct Annotations {
    unwind_info: bool,
}

impl Annotations {
    fn line(&self, out: &mut impl Write, annotation: &str) -> io::Result<()> {
        if self.unwind_info {
            writeln!(out, "\t{annotation}")?;
        }

        Ok(())
    }

    /// Start of a restore chain whose registers `first..=last` are in
    /// their slots.
    fn restore_start(&self, out: &mut impl Write, chain: Option<(Gpr, Gpr)>) -> io::Result<()> {
        self.line(out, ".cfi_startproc")?;
        self.line(out, &format!(".cfi_offset lr, {LINK_SLOT}"))?;

        for gpr in chain
            .into_iter()
            .flat_map(|(first, last)| registers(first, last))
        {
            self.line(
                out,
                &format!(".cfi_offset r{}, {}", gpr.number(), save_slot(gpr)),
            )?;
        }

        Ok(())
    }
}

/// Loads `gpr` from its slot; from then on it holds the caller's value.
fn restore(out: &mut impl Write, frame: &Annotations, gpr: Gpr) -> io::Result<()> {
    writeln!(out, "\tlwz {}, {}(1)", gpr.number(), save_slot(gpr))?;
    frame.line(out, &format!(".cfi_same_value r{}", gpr.number()))
}

pub fn save_name(range: SaveRange) -> String {
    format!(
        "__xbox360_savegprlr_{}_{}",
        range.first.number(),
        range.last.number()
    )
}

pub fn restore_name(range: Option<SaveRange>) -> String {
    match range {
        Some(range) => format!(
            "__xbox360_restgprlr_{}_{}",
            range.first.number(),
            range.last.number()
        ),
        None => "__xbox360_restlr".to_owned(),
    }
}

fn chain_index(last: Gpr) -> usize {
    usize::from(last == Gpr::R31)
}

fn registers(first: Gpr, last: Gpr) -> impl Iterator<Item = Gpr> {
    SaveRange { first, last }.registers()
}

fn entry(out: &mut impl Write, name: &str) -> io::Result<()> {
    writeln!(out, "\t.type\t{name},@function")?;
    writeln!(out, "{name}:")
}

fn restore_link(out: &mut impl Write) -> io::Result<()> {
    writeln!(out, "\tlwz 0, {LINK_SLOT}(1)")?;
    writeln!(out, "\tmtlr 0")?;
    writeln!(out, "\tblr")
}
