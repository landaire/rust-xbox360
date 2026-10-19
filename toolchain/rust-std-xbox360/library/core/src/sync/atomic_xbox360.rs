//! Protected atomic RMW/CAS for the Xbox 360 devkit.
//!
//! The hypervisor cannot preserve an `lwarx` reservation across an interrupt,
//! so `lwarx`/`stwcx.` must be bracketed by `mtmsrd` (disable external
//! interrupts) exactly the way existing titles do it, or the devkit kernel
//! traps them as "unprotected reservations". LLVM emits bare lwarx/stwcx, so
//! `core::sync::atomic`'s RMW/CAS helpers call into here instead on this target.
//!
//! All operations are full-barrier (the `Ordering` is ignored: conservative but
//! always correct). Atomic loads/stores stay as plain aligned accesses (they do
//! not use reservations) and are handled by the normal intrinsic path.
//!
//! Sizes 1, 2 and 4 go through a protected `lwarx`/`stwcx.` on the
//! containing word. Size 8 uses a protected `ldarx`/`stdcx.`: the CPU is
//! 64-bit, running 32-bit code, so doubleword reservations and `ld`/`std`
//! work, and 8-byte atomic loads and stores are single `ld`/`std`
//! instructions, which are single-copy atomic when aligned.

use crate::mem::size_of;

/// Protected compare-and-swap on an aligned 32-bit word. Stores `new` iff the
/// word equals `old`; returns the observed word.
///
/// `mtmsrd` disables external interrupts around the reservation (the hypervisor
/// cannot preserve it otherwise). Critically there is NO branch between `lwarx`
/// and `stwcx.` -- the devkit scanner rejects a reservation a branch can escape
/// -- so the compare is done branchlessly and `stwcx.` always runs (storing the
/// unchanged value on mismatch). The `mtmsrd` restore sits right after `stwcx.`,
/// before the retry branch, matching the recognized pattern.
#[inline]
unsafe fn word_cas(ptr: *mut u32, old: u32, new: u32) -> u32 {
    let seen: u32;
    unsafe {
        crate::arch::asm!(
            "sync",
            "mfmsr {s}",
            "2:",
            // Disable EE by loading r13 into MSR (L=1: only EE/RI change). r13 is
            // the per-thread block pointer, always in 0x80000000+ with bit 0x8000
            // clear, so this clears EE. The devkit scanner recognizes exactly this
            // `mtmsrd r13, 1` form as a protected reservation and leaves the
            // lwarx/stwcx alone; a computed register is flagged and trapped.
            "mtmsrd 13, 1",
            "lwarx  {seen}, 0, {p}",
            "cmpw   {seen}, {old}",
            "bne-   3f",                  // mismatch: release and finish
            "stwcx. {new}, 0, {p}",
            "mtmsrd {s}, 1",              // restore EE right after stwcx
            "bne-   2b",                  // stwcx failed (reservation lost): retry
            "b      4f",
            "3:",
            "stwcx. {seen}, 0, {p}",      // release the reservation (store unchanged)
            "mtmsrd {s}, 1",
            "4:",
            "isync",
            p = in(reg) ptr,
            old = in(reg) old,
            new = in(reg) new,
            s = out(reg) _,
            seen = out(reg) seen,
            options(nostack),
        );
    }
    seen
}

/// Protected compare-and-swap on an aligned doubleword, as `word_cas`. The
/// 64-bit values travel in 32-bit register halves and are joined for the
/// reservation.
#[inline]
unsafe fn doubleword_cas(ptr: *mut u64, old: u64, new: u64) -> u64 {
    let (seen_high, seen_low): (u32, u32);
    unsafe {
        crate::arch::asm!(
            "rldimi {old_low}, {old_high}, 32, 0",
            "rldimi {new_low}, {new_high}, 32, 0",
            "sync",
            "mfmsr {s}",
            "2:",
            "mtmsrd 13, 1",
            "ldarx  {seen}, 0, {p}",
            "cmpd   {seen}, {old_low}",
            "bne-   3f",
            "stdcx. {new_low}, 0, {p}",
            "mtmsrd {s}, 1",
            "bne-   2b",
            "b      4f",
            "3:",
            "stdcx. {seen}, 0, {p}",
            "mtmsrd {s}, 1",
            "4:",
            "isync",
            "srdi   {high}, {seen}, 32",
            "clrldi {low}, {seen}, 32",
            p = in(reg) ptr,
            old_high = in(reg) (old >> 32) as u32,
            old_low = inout(reg) old as u32 => _,
            new_high = in(reg) (new >> 32) as u32,
            new_low = inout(reg) new as u32 => _,
            s = out(reg) _,
            seen = out(reg) _,
            high = out(reg) seen_high,
            low = out(reg) seen_low,
            options(nostack),
        );
    }
    (u64::from(seen_high) << 32) | u64::from(seen_low)
}

/// Single-copy atomic load of an aligned doubleword with one `ld`.
#[inline]
pub(super) unsafe fn load_doubleword(ptr: *const u64) -> u64 {
    let (high, low): (u32, u32);
    unsafe {
        crate::arch::asm!(
            "ld     {value}, 0({p})",
            "srdi   {high}, {value}, 32",
            "clrldi {low}, {value}, 32",
            p = in(reg) ptr,
            value = out(reg) _,
            high = out(reg) high,
            low = out(reg) low,
            options(nostack, readonly, preserves_flags),
        );
    }
    (u64::from(high) << 32) | u64::from(low)
}

/// Single-copy atomic store of an aligned doubleword with one `std`.
#[inline]
pub(super) unsafe fn store_doubleword(ptr: *mut u64, value: u64) {
    unsafe {
        crate::arch::asm!(
            "rldimi {low}, {high}, 32, 0",
            "std    {low}, 0({p})",
            p = in(reg) ptr,
            high = in(reg) (value >> 32) as u32,
            low = inout(reg) value as u32 => _,
            options(nostack, preserves_flags),
        );
    }
}

/// Bytes of a doubleword atomic.
const DOUBLEWORD: usize = 8;

#[inline]
unsafe fn to_u64<T: Copy>(v: T) -> u64 {
    let mut value = 0u64;
    unsafe { crate::ptr::copy_nonoverlapping(&v as *const T as *const u8, &mut value as *mut u64 as *mut u8, DOUBLEWORD) };
    value
}

#[inline]
unsafe fn from_u64<T: Copy>(v: u64) -> T {
    let mut out = crate::mem::MaybeUninit::<T>::uninit();
    unsafe {
        crate::ptr::copy_nonoverlapping(&v as *const u64 as *const u8, out.as_mut_ptr() as *mut u8, DOUBLEWORD);
        out.assume_init()
    }
}

/// Doubleword read-modify-write with `op(current) -> new`; returns the old value.
#[inline]
unsafe fn rmw64<T: Copy>(dst: *mut T, op: impl Fn(u64) -> u64) -> T {
    let ptr = dst as *mut u64;
    loop {
        let current = unsafe { load_doubleword(ptr) };
        if unsafe { doubleword_cas(ptr, current, op(current)) } == current {
            return unsafe { from_u64::<T>(current) };
        }
    }
}

/// (aligned word ptr, bit shift of the field, value mask) for a `size`-byte
/// field at `addr`, big-endian.
#[inline]
fn field(addr: usize, size: usize) -> (*mut u32, u32, u32) {
    let aligned = (addr & !3) as *mut u32;
    let boff = addr & 3;
    let shift = ((4 - size - boff) * 8) as u32;
    let wmask: u32 = if size >= 4 { u32::MAX } else { (1u32 << (size * 8)) - 1 };
    (aligned, shift, wmask)
}

#[inline]
unsafe fn to_u32<T: Copy>(v: T) -> u32 {
    let size = size_of::<T>();
    let mut be = [0u8; 4];
    unsafe {
        crate::ptr::copy_nonoverlapping(
            &v as *const T as *const u8,
            be.as_mut_ptr().add(4 - size),
            size,
        );
    }
    u32::from_be_bytes(be)
}

#[inline]
unsafe fn from_u32<T: Copy>(v: u32) -> T {
    let size = size_of::<T>();
    let be = v.to_be_bytes();
    let mut out = crate::mem::MaybeUninit::<T>::uninit();
    unsafe {
        crate::ptr::copy_nonoverlapping(
            be.as_ptr().add(4 - size),
            out.as_mut_ptr() as *mut u8,
            size,
        );
        out.assume_init()
    }
}

/// Read-modify-write: `op(current_field_value) -> new_field_value`, both as the
/// field's value in the low bits of a u32. Returns the old value as `T`.
#[inline]
unsafe fn rmw<T: Copy>(dst: *mut T, op: impl Fn(u32) -> u32) -> T {
    let size = size_of::<T>();
    let (ap, shift, wmask) = field(dst as usize, size);
    loop {
        let word = unsafe { ap.read_volatile() };
        let cur = (word >> shift) & wmask;
        let next = op(cur) & wmask;
        let neww = (word & !(wmask << shift)) | (next << shift);
        if unsafe { word_cas(ap, word, neww) } == word {
            return unsafe { from_u32::<T>(cur) };
        }
    }
}

/// Sign-extend the low `size` bytes of `v` to `i32`.
#[inline]
fn sext(v: u32, size: usize) -> i32 {
    let bits = (size * 8) as u32;
    if bits >= 32 {
        v as i32
    } else {
        let sh = 32 - bits;
        ((v << sh) as i32) >> sh
    }
}

pub(super) unsafe fn swap<T: Copy>(dst: *mut T, val: T) -> T {
    if size_of::<T>() == DOUBLEWORD {
        let v = unsafe { to_u64(val) };
        return unsafe { rmw64(dst, |_| v) };
    }
    let v = unsafe { to_u32(val) };
    unsafe { rmw(dst, |_| v) }
}
pub(super) unsafe fn add<T: Copy, U: Copy>(dst: *mut T, val: U) -> T {
    if size_of::<T>() == DOUBLEWORD {
        let v = unsafe { to_u64(val) };
        return unsafe { rmw64(dst, |c| c.wrapping_add(v)) };
    }
    let v = unsafe { to_u32(val) };
    unsafe { rmw(dst, |c| c.wrapping_add(v)) }
}
pub(super) unsafe fn sub<T: Copy, U: Copy>(dst: *mut T, val: U) -> T {
    if size_of::<T>() == DOUBLEWORD {
        let v = unsafe { to_u64(val) };
        return unsafe { rmw64(dst, |c| c.wrapping_sub(v)) };
    }
    let v = unsafe { to_u32(val) };
    unsafe { rmw(dst, |c| c.wrapping_sub(v)) }
}
pub(super) unsafe fn and<T: Copy, U: Copy>(dst: *mut T, val: U) -> T {
    if size_of::<T>() == DOUBLEWORD {
        let v = unsafe { to_u64(val) };
        return unsafe { rmw64(dst, |c| c & v) };
    }
    let v = unsafe { to_u32(val) };
    unsafe { rmw(dst, |c| c & v) }
}
pub(super) unsafe fn nand<T: Copy, U: Copy>(dst: *mut T, val: U) -> T {
    if size_of::<T>() == DOUBLEWORD {
        let v = unsafe { to_u64(val) };
        return unsafe { rmw64(dst, |c| !(c & v)) };
    }
    let v = unsafe { to_u32(val) };
    unsafe { rmw(dst, |c| !(c & v)) }
}
pub(super) unsafe fn or<T: Copy, U: Copy>(dst: *mut T, val: U) -> T {
    if size_of::<T>() == DOUBLEWORD {
        let v = unsafe { to_u64(val) };
        return unsafe { rmw64(dst, |c| c | v) };
    }
    let v = unsafe { to_u32(val) };
    unsafe { rmw(dst, |c| c | v) }
}
pub(super) unsafe fn xor<T: Copy, U: Copy>(dst: *mut T, val: U) -> T {
    if size_of::<T>() == DOUBLEWORD {
        let v = unsafe { to_u64(val) };
        return unsafe { rmw64(dst, |c| c ^ v) };
    }
    let v = unsafe { to_u32(val) };
    unsafe { rmw(dst, |c| c ^ v) }
}
pub(super) unsafe fn max<T: Copy>(dst: *mut T, val: T) -> T {
    if size_of::<T>() == DOUBLEWORD {
        let v = unsafe { to_u64(val) };
        return unsafe { rmw64(dst, |c| if (c as i64) >= (v as i64) { c } else { v }) };
    }
    let size = size_of::<T>();
    let v = unsafe { to_u32(val) };
    unsafe { rmw(dst, |c| if sext(c, size) >= sext(v, size) { c } else { v }) }
}
pub(super) unsafe fn min<T: Copy>(dst: *mut T, val: T) -> T {
    if size_of::<T>() == DOUBLEWORD {
        let v = unsafe { to_u64(val) };
        return unsafe { rmw64(dst, |c| if (c as i64) <= (v as i64) { c } else { v }) };
    }
    let size = size_of::<T>();
    let v = unsafe { to_u32(val) };
    unsafe { rmw(dst, |c| if sext(c, size) <= sext(v, size) { c } else { v }) }
}
pub(super) unsafe fn umax<T: Copy>(dst: *mut T, val: T) -> T {
    if size_of::<T>() == DOUBLEWORD {
        let v = unsafe { to_u64(val) };
        return unsafe { rmw64(dst, |c| if c >= v { c } else { v }) };
    }
    let v = unsafe { to_u32(val) };
    unsafe { rmw(dst, |c| if c >= v { c } else { v }) }
}
pub(super) unsafe fn umin<T: Copy>(dst: *mut T, val: T) -> T {
    if size_of::<T>() == DOUBLEWORD {
        let v = unsafe { to_u64(val) };
        return unsafe { rmw64(dst, |c| if c <= v { c } else { v }) };
    }
    let v = unsafe { to_u32(val) };
    unsafe { rmw(dst, |c| if c <= v { c } else { v }) }
}

/// Compare-exchange on a `size`-byte field. Returns `(value, succeeded)`.
pub(super) unsafe fn compare_exchange<T: Copy>(dst: *mut T, old: T, new: T) -> (T, bool) {
    if size_of::<T>() == DOUBLEWORD {
        let (o, n) = unsafe { (to_u64(old), to_u64(new)) };
        let seen = unsafe { doubleword_cas(dst as *mut u64, o, n) };
        return (unsafe { from_u64::<T>(seen) }, seen == o);
    }
    let size = size_of::<T>();
    let (ap, shift, wmask) = field(dst as usize, size);
    let o = unsafe { to_u32(old) };
    let n = unsafe { to_u32(new) };
    loop {
        let word = unsafe { ap.read_volatile() };
        let cur = (word >> shift) & wmask;
        if cur != o {
            return (unsafe { from_u32::<T>(cur) }, false);
        }
        let neww = (word & !(wmask << shift)) | (n << shift);
        if unsafe { word_cas(ap, word, neww) } == word {
            return (unsafe { from_u32::<T>(o) }, true);
        }
    }
}
