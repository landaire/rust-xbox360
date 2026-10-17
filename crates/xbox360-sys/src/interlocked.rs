//! Interlocked operations in the form the kernel loader accepts.
//!
//! The loader reports every `lwarx`/`ldarx` that does not directly follow
//! `mtmsrd r13, 1` and every `stwcx.`/`stdcx.` that is not directly followed
//! by `mtmsrd rS, 1`. Each reservation here runs with external interrupts
//! disabled and restores the previous MSR after the store.
//!
//! The 64-bit operations take and return values in SysV register pairs and
//! join them into one doubleword.
//!
//! Each function documents its memory ordering. None orders later accesses
//! after the operation: a caller that needs acquire ordering issues
//! `isync` (after a dependent branch) or `lwsync` after the call, and a
//! caller of a relaxed operation that needs release ordering issues
//! `lwsync` before it.

/// Compare the word at `target` with `expected` and store `new` on a match.
/// Returns the observed word. A mismatch stores the observed word back to
/// end the reservation.
///
/// # Ordering
///
/// Release: a leading `lwsync` orders earlier loads and stores before the
/// operation. No barrier follows it.
///
/// # Safety
///
/// `target` must be valid for reads and writes and 4-byte aligned.
#[unsafe(naked)]
pub unsafe extern "C" fn compare_exchange32(target: *mut u32, expected: u32, new: u32) -> u32 {
    core::arch::naked_asm!(
        "lwsync",
        "1:",
        "mfmsr 9",
        "mtmsrd 13, 1",
        "lwarx 10, 0, 3",
        "cmpw 10, 4",
        "bne 2f",
        "stwcx. 5, 0, 3",
        "mtmsrd 9, 1",
        "bne 1b",
        "b 3f",
        "2:",
        "stwcx. 10, 0, 3",
        "mtmsrd 9, 1",
        "3:",
        "mr 3, 10",
        "blr",
    )
}

/// Compare the doubleword at `target` with `expected` and store `new` on a
/// match. Returns the observed doubleword. A mismatch stores the observed
/// value back to end the reservation.
///
/// # Ordering
///
/// Release: a leading `lwsync` orders earlier loads and stores before the
/// operation. No barrier follows it.
///
/// # Safety
///
/// `target` must be valid for reads and writes and 8-byte aligned.
#[unsafe(naked)]
pub unsafe extern "C" fn compare_exchange64(target: *mut u64, expected: u64, new: u64) -> u64 {
    core::arch::naked_asm!(
        "lwsync",
        // Join the word pairs.
        "rldimi 6, 5, 32, 0",
        "rldimi 8, 7, 32, 0",
        "1:",
        "mfmsr 9",
        "mtmsrd 13, 1",
        "ldarx 10, 0, 3",
        "cmpd 10, 6",
        "bne 2f",
        "stdcx. 8, 0, 3",
        "mtmsrd 9, 1",
        "bne 1b",
        "b 3f",
        "2:",
        "stdcx. 10, 0, 3",
        "mtmsrd 9, 1",
        "3:",
        // The high word into r3.
        "rldicl 3, 10, 32, 32",
        "mr 4, 10",
        "blr",
    )
}

/// Add `delta` to the word at `target`. Returns the previous value.
///
/// # Ordering
///
/// Relaxed: no barrier.
///
/// # Safety
///
/// `target` must be valid for reads and writes and 4-byte aligned.
#[unsafe(naked)]
pub unsafe extern "C" fn fetch_add32(target: *mut u32, delta: u32) -> u32 {
    core::arch::naked_asm!(
        "1:",
        "mfmsr 9",
        "mtmsrd 13, 1",
        "lwarx 10, 0, 3",
        "add 11, 10, 4",
        "stwcx. 11, 0, 3",
        "mtmsrd 9, 1",
        "bne 1b",
        "mr 3, 10",
        "blr",
    )
}

/// Add `delta` to the doubleword at `target`, wrapping on overflow.
/// Returns the previous value.
///
/// # Ordering
///
/// Relaxed: no barrier.
///
/// # Safety
///
/// `target` must be valid for reads and writes and 8-byte aligned.
#[unsafe(naked)]
pub unsafe extern "C" fn fetch_add64(target: *mut u64, delta: u64) -> u64 {
    core::arch::naked_asm!(
        // Join the word pair.
        "rldimi 6, 5, 32, 0",
        "1:",
        "mfmsr 9",
        "mtmsrd 13, 1",
        "ldarx 10, 0, 3",
        "add 11, 10, 6",
        "stdcx. 11, 0, 3",
        "mtmsrd 9, 1",
        "bne 1b",
        // The high word into r3.
        "rldicl 3, 10, 32, 32",
        "mr 4, 10",
        "blr",
    )
}

/// Store `value` at `target`. Returns the previous value.
///
/// # Ordering
///
/// Relaxed: no barrier.
///
/// # Safety
///
/// `target` must be valid for reads and writes and 4-byte aligned.
#[unsafe(naked)]
pub unsafe extern "C" fn exchange32(target: *mut u32, value: u32) -> u32 {
    core::arch::naked_asm!(
        "1:",
        "mfmsr 9",
        "mtmsrd 13, 1",
        "lwarx 10, 0, 3",
        "stwcx. 4, 0, 3",
        "mtmsrd 9, 1",
        "bne 1b",
        "mr 3, 10",
        "blr",
    )
}

/// Load the doubleword at `address` in one access.
///
/// # Ordering
///
/// Relaxed: no barrier.
///
/// # Safety
///
/// `address` must be valid for reads and 8-byte aligned.
#[unsafe(naked)]
pub unsafe extern "C" fn load64(address: *const u64) -> u64 {
    core::arch::naked_asm!("ld 4, 0(3)", "rldicl 3, 4, 32, 32", "blr",)
}

/// Store `value` at `address` in one access.
///
/// # Ordering
///
/// Relaxed: no barrier.
///
/// # Safety
///
/// `address` must be valid for writes and 8-byte aligned.
#[unsafe(naked)]
pub unsafe extern "C" fn store64(address: *mut u64, value: u64) {
    core::arch::naked_asm!("rldimi 6, 5, 32, 0", "std 6, 0(3)", "blr",)
}
