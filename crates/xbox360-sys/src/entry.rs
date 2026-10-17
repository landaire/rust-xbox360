/// Call the function in r12 with r3-r10 unchanged and the full 64-bit
/// r14-r31 preserved. The CPU model of the target has no 64-bit mnemonics,
/// so `std`/`ld` are encoded directly.
///
/// # Safety
///
/// Enter it only with `b` from a stub that loads r12 with the target, as
/// [`bridged!`](crate::bridged!) does. It is not callable from Rust.
#[cfg(any(target_os = "xbox360", xbox360_docs))]
#[unsafe(naked)]
pub unsafe extern "C" fn entry_bridge() {
    core::arch::naked_asm!(
        "stwu 1, -0xa0(1)",
        "mflr 0",
        "stw 0, 0x8(1)",
        ".irp reg, 14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31",
        // std \reg, 0x10 + 8 * (\reg - 14)(r1)
        ".long (62 << 26) | (\\reg << 21) | (1 << 16) | (0x10 + 8 * (\\reg - 14))",
        ".endr",
        "mtctr 12",
        "bctrl",
        ".irp reg, 14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31",
        // ld \reg, 0x10 + 8 * (\reg - 14)(r1)
        ".long (58 << 26) | (\\reg << 21) | (1 << 16) | (0x10 + 8 * (\\reg - 14))",
        ".endr",
        "lwz 0, 0x8(1)",
        "mtlr 0",
        "addi 1, 1, 0xa0",
        "blr",
    )
}

/// Define `$entry`, an entry for console-ABI callers that runs `$target`
/// (an `extern "C" fn` with at most eight 32-bit arguments and a 32-bit or
/// no result) behind [`entry_bridge`], or behind `bridge`, which must end by
/// branching to [`entry_bridge`] with r12 unchanged.
///
/// ```ignore
/// xbox360_sys::bridged!(#[unsafe(no_mangle)] pub _start, title_main);
/// ```
#[macro_export]
macro_rules! bridged {
    ($(#[$meta:meta])* $vis:vis $entry:ident, $target:path, bridge = $bridge:path) => {
        $(#[$meta])*
        #[unsafe(naked)]
        $vis unsafe extern "C" fn $entry() {
            ::core::arch::naked_asm!(
                "lis 12, {target}@ha",
                "addi 12, 12, {target}@l",
                "b {bridge}",
                target = sym $target,
                bridge = sym $bridge,
            )
        }
    };
    ($(#[$meta:meta])* $vis:vis $entry:ident, $target:path) => {
        $crate::bridged!($(#[$meta])* $vis $entry, $target, bridge = $crate::entry_bridge);
    };
}
