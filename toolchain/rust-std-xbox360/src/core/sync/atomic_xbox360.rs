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
//! Only sizes 1, 2, and 4 occur here: `max-atomic-width` is 32, so 64-bit
//! atomics never reach these helpers.

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
            "ori   {t}, {s}, 0x8000",
            "xori  {t}, {t}, 0x8000",     // EE cleared
            "2:",
            "mtmsrd {t}, 1",              // disable EE (L=1: EE/RI only)
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
            t = out(reg) _,
            seen = out(reg) seen,
            options(nostack),
        );
    }
    seen
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
    let v = unsafe { to_u32(val) };
    unsafe { rmw(dst, |_| v) }
}
pub(super) unsafe fn add<T: Copy, U: Copy>(dst: *mut T, val: U) -> T {
    let v = unsafe { to_u32(val) };
    unsafe { rmw(dst, |c| c.wrapping_add(v)) }
}
pub(super) unsafe fn sub<T: Copy, U: Copy>(dst: *mut T, val: U) -> T {
    let v = unsafe { to_u32(val) };
    unsafe { rmw(dst, |c| c.wrapping_sub(v)) }
}
pub(super) unsafe fn and<T: Copy, U: Copy>(dst: *mut T, val: U) -> T {
    let v = unsafe { to_u32(val) };
    unsafe { rmw(dst, |c| c & v) }
}
pub(super) unsafe fn nand<T: Copy, U: Copy>(dst: *mut T, val: U) -> T {
    let v = unsafe { to_u32(val) };
    unsafe { rmw(dst, |c| !(c & v)) }
}
pub(super) unsafe fn or<T: Copy, U: Copy>(dst: *mut T, val: U) -> T {
    let v = unsafe { to_u32(val) };
    unsafe { rmw(dst, |c| c | v) }
}
pub(super) unsafe fn xor<T: Copy, U: Copy>(dst: *mut T, val: U) -> T {
    let v = unsafe { to_u32(val) };
    unsafe { rmw(dst, |c| c ^ v) }
}
pub(super) unsafe fn max<T: Copy>(dst: *mut T, val: T) -> T {
    let size = size_of::<T>();
    let v = unsafe { to_u32(val) };
    unsafe { rmw(dst, |c| if sext(c, size) >= sext(v, size) { c } else { v }) }
}
pub(super) unsafe fn min<T: Copy>(dst: *mut T, val: T) -> T {
    let size = size_of::<T>();
    let v = unsafe { to_u32(val) };
    unsafe { rmw(dst, |c| if sext(c, size) <= sext(v, size) { c } else { v }) }
}
pub(super) unsafe fn umax<T: Copy>(dst: *mut T, val: T) -> T {
    let v = unsafe { to_u32(val) };
    unsafe { rmw(dst, |c| if c >= v { c } else { v }) }
}
pub(super) unsafe fn umin<T: Copy>(dst: *mut T, val: T) -> T {
    let v = unsafe { to_u32(val) };
    unsafe { rmw(dst, |c| if c <= v { c } else { v }) }
}

/// Compare-exchange on a `size`-byte field. Returns `(value, succeeded)`.
pub(super) unsafe fn compare_exchange<T: Copy>(dst: *mut T, old: T, new: T) -> (T, bool) {
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
