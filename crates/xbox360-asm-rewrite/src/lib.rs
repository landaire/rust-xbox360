//! Replaces the inline callee-saved GPR saves and restores that LLVM emits
//! in prologues and epilogues of Xbox 360 code with calls to shared
//! routines, in the manner of the stock compiler's `__savegprlr_N` and
//! `__restgprlr_N`.
//!
//! The input is the assembly of the final LTO module (`rustc --emit=asm`).
//! [`analysis`] documents the rules, [`labels`] finds the local labels
//! that control can enter, and [`relative`] names relative branch targets
//! with labels; [`audit`] reapplies the rules to the disassembly of the
//! linked image. [`reservation`] applies the kernel loader's check for
//! unprotected reservations to the linked image.

pub mod analysis;
pub mod audit;
pub mod helpers;
pub mod isa;
pub mod labels;
pub mod line;
pub mod relative;
pub mod reservation;
pub mod rewrite;

#[cfg(test)]
mod tests;
