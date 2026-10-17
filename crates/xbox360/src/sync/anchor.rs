use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;

/// Run `setup` once for the current address of an object, recorded in
/// `anchor`. Other users of the object at the same address wait for it.
pub(super) fn anchor_at(anchor: &AtomicU32, address: u32, setup: impl FnOnce()) {
    // Anchor value while one thread sets up an object. Object addresses are
    // 4-byte aligned, so it never equals one.
    const ANCHOR_BUSY: u32 = 1;

    loop {
        let current = anchor.load(Ordering::Acquire);
        if current == address {
            return;
        }

        if current == ANCHOR_BUSY {
            core::hint::spin_loop();
            continue;
        }

        if anchor
            .compare_exchange(current, ANCHOR_BUSY, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            setup();
            anchor.store(address, Ordering::Release);
            return;
        }
    }
}
