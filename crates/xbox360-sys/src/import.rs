use core::fmt;
use core::marker::PhantomData;
use core::num::NonZeroU16;
use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;

/// An export ordinal, as `XexGetProcedureAddress` takes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ordinal(NonZeroU16);

impl Ordinal {
    /// `None` for 0, which no export has.
    pub const fn new(ordinal: u16) -> Option<Self> {
        match NonZeroU16::new(ordinal) {
            Some(ordinal) => Some(Self(ordinal)),
            None => None,
        }
    }

    pub const fn get(self) -> u16 {
        self.0.get()
    }
}

/// A data import slot. Before any code of the module runs, the loader
/// replaces the import record in the slot with the address of the export,
/// a `T` that the kernel keeps for the life of the system. Safe code can
/// only read the slot.
#[repr(transparent)]
pub struct ImportedVariable<T: 'static> {
    address: AtomicU32,
    _export: PhantomData<&'static T>,
}

impl<T> ImportedVariable<T>
where
    T: Sync + 'static,
{
    /// The export. `None` when the loader left the import unresolved:
    /// Xenia stores 0xDxxxBEEF in the slots of exports it does not provide,
    /// 0xF00DF00D for an ordinal it does not list, and 0xDEADC0DE for a
    /// function export imported as a variable.
    pub fn get(&self) -> Option<&'static T> {
        let address = resolved_address(self.address.load(Ordering::Relaxed))?;

        // SAFETY: the loader stored the address of the export, a `T` that
        // the kernel keeps for the life of the system.
        Some(unsafe { &*core::ptr::with_exposed_provenance::<T>(address as usize) })
    }

    /// The export, without the check of [`ImportedVariable::get`]. For code
    /// that checks the import once, for example at module start, and then
    /// reads it often.
    ///
    /// # Safety
    ///
    /// The loader must have resolved the import: [`ImportedVariable::get`]
    /// returns `Some` for it. On the console kernel the loader resolves
    /// every import it accepts.
    #[inline]
    pub unsafe fn get_unchecked(&self) -> &'static T {
        let address = self.address.load(Ordering::Relaxed);

        // SAFETY: the caller guarantees that the loader stored the address
        // of the export, a `T` that the kernel keeps for the life of the
        // system.
        unsafe { &*core::ptr::with_exposed_provenance::<T>(address as usize) }
    }
}

impl<T> fmt::Debug for ImportedVariable<T>
where
    T: 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let address = self.address.load(Ordering::Relaxed);

        write!(f, "ImportedVariable({address:#010x})")
    }
}

/// Define an import: a stub with the signature callers use, and the thunk it
/// calls. Arguments pass through in r3-r10 to the resolved export.
/// Stack-passed arguments are not forwarded, so an import takes at most
/// eight arguments. [`import_thunk_stack!`](crate::import_thunk_stack!) takes
/// nine or ten.
///
/// ```ignore
/// xbox360_sys::import_thunk!(
///     /// Writes a formatted string with two values to the debug output.
///     pub DbgPrintTwo(format: *const u8, first: u32, second: u32) -> u32,
///     lib = xbox360_sys::lib_index::XBOXKRNL,
///     ordinal = 0x003
/// );
///
/// xbox360_sys::import_thunk!(
///     /// Returns the timebase frequency.
///     pub safe KeQueryPerformanceFrequency() -> u32,
///     lib = xbox360_sys::lib_index::XBOXKRNL,
///     ordinal = 0x083
/// );
/// ```
///
/// The stub is an `unsafe extern "C" fn`. With `safe` after the visibility,
/// as in an `unsafe extern` block, it is a safe `extern "C" fn`. Declare an
/// import `safe` only when no argument is an address, handle or object that
/// the export uses or releases, and the export has no effect that can break
/// the memory safety of other code. Declare an export that does not return
/// with `-> !`. Its stub traps if the export returns anyway.
///
/// Every argument and the result must be 32 bits or narrower. The macro
/// rejects a wider argument, a library index above 0xff and an ordinal
/// above 0xffff. SysV passes a 64-bit value in a register pair, while the
/// console callee expects it in one 64-bit register, so such an export
/// needs a stub that joins the pair. This crate has one for each export it
/// declares with a 64-bit argument.
///
/// The import exists only for `target_os = "xbox360"`. On other targets
/// the macro expands to nothing, so a module can declare imports next to
/// types that host code uses.
///
/// Each record goes in its own section, `.xthunks.<module path>::<name>`,
/// so that `--gc-sections` drops the records of imports nobody calls, also
/// when two crates declare an import with the same name. A linker script
/// places them with `*(.xthunks .xthunks.*)`.
#[macro_export]
macro_rules! import_thunk {
    ($(#[$meta:meta])* $vis:vis safe $name:ident($($arg:ident: $argty:ty),* $(,)?) -> !, lib = $lib:expr, ordinal = $ord:expr) => {
        $crate::__import_stub!(@check_registers $lib, $ord, [$($arg: $argty),*]);
        $crate::__import_stub!(@registers_diverging [] [$(#[$meta])*] [$vis] [] $name($($arg: $argty),*), $lib, $ord);
    };
    ($(#[$meta:meta])* $vis:vis safe $name:ident($($arg:ident: $argty:ty),* $(,)?) -> $ret:ty, lib = $lib:expr, ordinal = $ord:expr) => {
        $crate::__import_stub!(@check_registers $lib, $ord, [$($arg: $argty),*]);
        $crate::__import_stub!(@registers_returning [] [$(#[$meta])*] [$vis] [] $name($($arg: $argty),*) -> $ret, $lib, $ord);
    };
    ($(#[$meta:meta])* $vis:vis $name:ident($($arg:ident: $argty:ty),* $(,)?) -> !, lib = $lib:expr, ordinal = $ord:expr) => {
        $crate::__import_stub!(@check_registers $lib, $ord, [$($arg: $argty),*]);
        $crate::__import_stub!(@registers_diverging [] [$(#[$meta])*] [$vis] [unsafe] $name($($arg: $argty),*), $lib, $ord);
    };
    ($(#[$meta:meta])* $vis:vis $name:ident($($arg:ident: $argty:ty),* $(,)?) -> $ret:ty, lib = $lib:expr, ordinal = $ord:expr) => {
        $crate::__import_stub!(@check_registers $lib, $ord, [$($arg: $argty),*]);
        $crate::__import_stub!(@registers_returning [] [$(#[$meta])*] [$vis] [unsafe] $name($($arg: $argty),*) -> $ret, $lib, $ord);
    };
}

/// Define an import that takes nine or ten 32-bit arguments. SysV passes the
/// last two on the stack at sp+8 and sp+0xc. The console callee reads them
/// from the 8-byte parameter slots at sp+0x50 and sp+0x58 of its caller
/// frame. The forms and rules of [`import_thunk!`] apply.
#[macro_export]
macro_rules! import_thunk_stack {
    ($(#[$meta:meta])* $vis:vis safe $name:ident($($arg:ident: $argty:ty),* $(,)?) -> !, lib = $lib:expr, ordinal = $ord:expr) => {
        $crate::__import_stub!(@check_stack $lib, $ord, [$($arg: $argty),*]);
        $crate::__import_stub!(@stack ["trap"] [$(#[$meta])*] [$vis] [] $name($($arg: $argty),*) -> !, $lib, $ord);
    };
    ($(#[$meta:meta])* $vis:vis safe $name:ident($($arg:ident: $argty:ty),* $(,)?) -> $ret:ty, lib = $lib:expr, ordinal = $ord:expr) => {
        $crate::__import_stub!(@check_stack $lib, $ord, [$($arg: $argty),*]);
        $crate::__import_stub!(@stack ["lwz 0, 0x74(1)", "mtlr 0", "addi 1, 1, 0x70", "blr"] [$(#[$meta])*] [$vis] [] $name($($arg: $argty),*) -> $ret, $lib, $ord);
    };
    ($(#[$meta:meta])* $vis:vis $name:ident($($arg:ident: $argty:ty),* $(,)?) -> !, lib = $lib:expr, ordinal = $ord:expr) => {
        $crate::__import_stub!(@check_stack $lib, $ord, [$($arg: $argty),*]);
        $crate::__import_stub!(@stack ["trap"] [$(#[$meta])*] [$vis] [unsafe] $name($($arg: $argty),*) -> !, $lib, $ord);
    };
    ($(#[$meta:meta])* $vis:vis $name:ident($($arg:ident: $argty:ty),* $(,)?) -> $ret:ty, lib = $lib:expr, ordinal = $ord:expr) => {
        $crate::__import_stub!(@check_stack $lib, $ord, [$($arg: $argty),*]);
        $crate::__import_stub!(@stack ["lwz 0, 0x74(1)", "mtlr 0", "addi 1, 1, 0x70", "blr"] [$(#[$meta])*] [$vis] [unsafe] $name($($arg: $argty),*) -> $ret, $lib, $ord);
    };
}

/// Define an import whose export takes a 64-bit argument in one register.
/// `prologue` joins the SysV register pair with `rldimi` and moves the
/// later arguments to the registers the export reads them from. Then the
/// stub calls the thunk as [`import_thunk!`] does.
macro_rules! import_adapter {
    ($(#[$meta:meta])* $vis:vis $name:ident($($arg:ident: $argty:ty),* $(,)?) -> $ret:ty, lib = $lib:expr, ordinal = $ord:expr, prologue = [$($prologue:literal),* $(,)?]) => {
        $crate::__import_stub!(@check_record $lib, $ord);
        $crate::__import_stub!(@registers_returning [$($prologue),*] [$(#[$meta])*] [$vis] [unsafe] $name($($arg: $argty),*) -> $ret, $lib, $ord);
    };
}

/// Stubs and checks of [`import_thunk!`] and [`import_thunk_stack!`].
#[doc(hidden)]
#[macro_export]
macro_rules! __import_stub {
    (@safety_doc unsafe) => {
        "\n\n# Safety\n\nThe arguments and the calling context must meet the requirements of the export: \
         each address is valid for the accesses the export makes for as long as it uses it, \
         and each handle or object is one that the caller may use or release as the export does."
    };
    (@check_record $lib:expr, $ord:expr) => {
        $crate::__on_console! {
            const _: () = {
                ::core::assert!(($lib as u32) <= 0xff, "library index above 0xff");
                ::core::assert!(($ord as u32) <= 0xffff, "ordinal above 0xffff");
            };
        }
    };
    (@check_registers $lib:expr, $ord:expr, [$($arg:ident: $argty:ty),*]) => {
        $crate::__import_stub!(@check_record $lib, $ord);
        $crate::__on_console! {
            const _: () = {
                ::core::assert!(
                    <[&str]>::len(&[$(::core::stringify!($arg)),*]) <= 8,
                    "more than eight arguments; use import_thunk_stack!"
                );
                $(::core::assert!(::core::mem::size_of::<$argty>() <= 4, "argument wider than 32 bits");)*
            };
        }
    };
    (@check_stack $lib:expr, $ord:expr, [$($arg:ident: $argty:ty),*]) => {
        $crate::__import_stub!(@check_record $lib, $ord);
        $crate::__on_console! {
            const _: () = {
                ::core::assert!(
                    ::core::matches!(<[&str]>::len(&[$(::core::stringify!($arg)),*]), 9 | 10),
                    "import_thunk_stack! takes nine or ten arguments"
                );
                $(::core::assert!(::core::mem::size_of::<$argty>() <= 4, "argument wider than 32 bits");)*
            };
        }
    };
    (@registers_returning [$($prologue:literal),*] [$($attr:tt)*] [$vis:vis] [$($unsafety:tt)?] $name:ident($($arg:ident: $argty:ty),*) -> $ret:ty, $lib:expr, $ord:expr) => {
        $crate::__import_stub!(@registers [$($prologue),*] ["lwz 0, 0x54(1)", "mtlr 0", "addi 1, 1, 0x50", "blr"] [$($attr)*] [$vis] [$($unsafety)?] $name($($arg: $argty),*) -> $ret, $lib, $ord);
    };
    // A diverging export that returns anyway, as Xenia's KeBugCheck does,
    // reaches the trap instead of code the compiler treats as unreachable.
    (@registers_diverging [$($prologue:literal),*] [$($attr:tt)*] [$vis:vis] [$($unsafety:tt)?] $name:ident($($arg:ident: $argty:ty),*), $lib:expr, $ord:expr) => {
        $crate::__import_stub!(@registers [$($prologue),*] ["trap"] [$($attr)*] [$vis] [$($unsafety)?] $name($($arg: $argty),*) -> !, $lib, $ord);
    };
    (@registers [$($prologue:literal),*] [$($epilogue:literal),*] [$($attr:tt)*] [$vis:vis] [$($unsafety:tt)?] $name:ident($($arg:ident: $argty:ty),*) -> $ret:ty, $lib:expr, $ord:expr) => {
        $crate::__on_console! {
            $($attr)*
            $(#[doc = $crate::__import_stub!(@safety_doc $unsafety)])?
            #[unsafe(naked)]
            $vis $($unsafety)? extern "C" fn $name($($arg: $argty),*) -> $ret {
                ::core::arch::naked_asm!(
                    $($prologue,)*
                    "stwu 1, -0x50(1)",
                    "mflr 0",
                    "stw 0, 0x54(1)",
                    "bl 2f",
                    $($epilogue,)*
                    ::core::concat!(".pushsection \".xthunks.", ::core::module_path!(), "::", ::core::stringify!($name), "\", \"ax\""),
                    ".balign 4",
                    "2:",
                    ".long {record}",
                    ".long 0x60000000", // nop
                    ".long 0x60000000", // nop
                    ".long 0x4e800020", // blr
                    ".popsection",
                    record = const (1u32 << 24) | (($lib as u32) << 16) | ($ord as u32),
                )
            }
        }
    };
    (@stack [$($epilogue:literal),*] [$($attr:tt)*] [$vis:vis] [$($unsafety:tt)?] $name:ident($($arg:ident: $argty:ty),*) -> $ret:ty, $lib:expr, $ord:expr) => {
        $crate::__on_console! {
            $($attr)*
            $(#[doc = $crate::__import_stub!(@safety_doc $unsafety)])?
            #[unsafe(naked)]
            $vis $($unsafety)? extern "C" fn $name($($arg: $argty),*) -> $ret {
                ::core::arch::naked_asm!(
                    "stwu 1, -0x70(1)",
                    "mflr 0",
                    "stw 0, 0x74(1)",
                    "li 12, 0",
                    "lwz 11, 0x78(1)",
                    "stw 12, 0x50(1)",
                    "stw 11, 0x54(1)",
                    "lwz 11, 0x7c(1)",
                    "stw 12, 0x58(1)",
                    "stw 11, 0x5c(1)",
                    "bl 2f",
                    $($epilogue,)*
                    ::core::concat!(".pushsection \".xthunks.", ::core::module_path!(), "::", ::core::stringify!($name), "\", \"ax\""),
                    ".balign 4",
                    "2:",
                    ".long {record}",
                    ".long 0x60000000", // nop
                    ".long 0x60000000", // nop
                    ".long 0x4e800020", // blr
                    ".popsection",
                    record = const (1u32 << 24) | (($lib as u32) << 16) | ($ord as u32),
                )
            }
        }
    };
}

/// Expand to the given items when this crate is built for the console
/// target (or for its documentation with `--cfg xbox360_docs`), and to
/// nothing otherwise. The macros of this crate gate what they generate
/// through it, so the crate that invokes them needs no cfg of its own.
#[cfg(any(target_os = "xbox360", xbox360_docs))]
#[doc(hidden)]
#[macro_export]
macro_rules! __on_console {
    ($($item:tt)*) => {
        $($item)*
    };
}

#[cfg(not(any(target_os = "xbox360", xbox360_docs)))]
#[doc(hidden)]
#[macro_export]
macro_rules! __on_console {
    ($($item:tt)*) => {};
}

/// Declare a data import `$name: $type`: an [`ImportedVariable`] slot that
/// the loader replaces with the address of the export.
///
/// The slot is defined in assembly and reached through an `extern` static,
/// so LLVM cannot treat a slot that no code writes as a constant and fold
/// its record value into the reads. Each slot goes in its own section,
/// `.xvars.<module path>::<name>`, so that `--gc-sections` drops the slots
/// nobody reads. A linker script places the slots in a writable section
/// with `*(.xvars .xvars.*)`.
///
/// The slot symbol is `__xbox360_import_<name>`, so `name` must be the
/// export name. Separate objects that declare it, such as the copies of
/// this crate that std and a title link, each define it weak, and the
/// linker keeps one. In one assembly unit, such as a fat LTO module, only
/// the first declaration defines it.
///
/// The static is `unsafe` to read. With `safe` after the visibility, as in
/// an `unsafe extern` block, it is safe to read, and [`ImportedVariable`]
/// then hands out the export as a `&'static $type` without `unsafe`.
/// Declare an import `safe` only when `$type` describes the export as the
/// kernel keeps it: a valid value for every bit pattern the kernel stores,
/// and an atomic for every field that the kernel or another processor
/// changes.
///
/// ```ignore
/// xbox360_sys::import_variable!(
///     /// Version of the running kernel.
///     pub safe XboxKrnlVersion: xbox360_sys::xboxkrnl::KernelVersion,
///     lib = xbox360_sys::lib_index::XBOXKRNL,
///     ordinal = 0x158
/// );
/// ```
///
/// As with [`import_thunk!`], the slot exists only on the console target.
#[macro_export]
macro_rules! import_variable {
    ($(#[$meta:meta])* $vis:vis safe $name:ident: $type:ty, lib = $lib:expr, ordinal = $ord:expr) => {
        $crate::import_variable!(@slot [$(#[$meta])*] [$vis] [safe] $name: $type, $lib, $ord);
    };
    ($(#[$meta:meta])* $vis:vis $name:ident: $type:ty, lib = $lib:expr, ordinal = $ord:expr) => {
        $crate::import_variable!(@slot [$(#[$meta])*] [$vis] [] $name: $type, $lib, $ord);
    };
    (@slot [$($attr:tt)*] [$vis:vis] [$($safety:ident)?] $name:ident: $type:ty, $lib:expr, $ord:expr) => {
        $crate::__import_stub!(@check_record $lib, $ord);

        $crate::__on_console! {
            ::core::arch::global_asm!(
                ::core::concat!(".ifndef __xbox360_import_", ::core::stringify!($name)),
                ::core::concat!(".pushsection \".xvars.", ::core::module_path!(), "::", ::core::stringify!($name), "\", \"aw\""),
                ".balign 4",
                ::core::concat!(".weak __xbox360_import_", ::core::stringify!($name)),
                ::core::concat!(".type __xbox360_import_", ::core::stringify!($name), ", @object"),
                ::core::concat!(".size __xbox360_import_", ::core::stringify!($name), ", 4"),
                ::core::concat!("__xbox360_import_", ::core::stringify!($name), ":"),
                ".long {record}",
                ".popsection",
                ".endif",
                record = const (($lib as u32) << 16) | ($ord as u32),
            );

            unsafe extern "C" {
                $($attr)*
                #[link_name = ::core::concat!("__xbox360_import_", ::core::stringify!($name))]
                $vis $($safety)? static $name: $crate::ImportedVariable<$type>;
            }
        }
    };
}

/// `address` unless it is a value Xenia leaves in the slot of a data import
/// it does not resolve.
const fn resolved_address(address: u32) -> Option<u32> {
    if address & 0xf000_ffff == 0xd000_beef || address == 0xf00d_f00d || address == 0xdead_c0de {
        None
    } else {
        Some(address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unresolved_data_import_has_no_address() {
        assert_eq!(resolved_address(0xd000_beef), None);
        assert_eq!(resolved_address(0xd123_beef), None);
        assert_eq!(resolved_address(0x801f_beef), Some(0x801f_beef));
        assert_eq!(resolved_address(0xf00d_f00d), None);
        assert_eq!(resolved_address(0xdead_c0de), None);
    }
}
