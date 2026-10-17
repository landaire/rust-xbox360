//! Raw bindings to the Xbox 360 kernel and XAM for the
//! `powerpc-unknown-xbox360` target: imports, exports, entries for callers
//! that use the console ABI, the interlocked operations the kernel loader
//! accepts, and the types and constants of the import signatures.
//!
//! Each import is a 16-byte thunk in its own `.xthunks.*` section. The first
//! big-endian DWORD is the import record `(type<<24 | lib<<16 | ordinal)`.
//! `xex-link` collects these into the XEX `ImportLibraries` table, and at load
//! time Xenia (and a real console) rewrites the thunk to a syscall trap. The
//! remaining DWORDs (`nop; nop; blr`) are only a fallback if never patched.
//!
//! Console callees home r3-r10 into the caller frame at sp+0x10..sp+0x50.
//! SysV frames do not reserve that area, so callers reach each thunk through
//! a stub that allocates it.
//!
//! Console callers expect r14-r31 preserved in full 64 bits. Code for this
//! target saves only their low words, so code that the kernel, the loader or
//! another module calls (an entry point, an export, a thread start routine,
//! a callback) enters through [`entry_bridge`], which [`bridged!`] and
//! [`export!`] arrange.
//!
//! Types, constants, structure layouts and [`status`] compile on every
//! target, so host code can share them. Imports, the functions of
//! [`processor`], [`mmio`], [`interlocked`] and the entry bridge exist only
//! for `target_os = "xbox360"`. Documentation builds them on another
//! PowerPC target with `--cfg xbox360_docs`.
//!
//! Every type in an import signature is 32 bits wide or narrower and passes
//! like the C integer of its width: enums are `#[repr(u32)]` (or the C
//! width), and newtypes are `#[repr(transparent)]`. Values the kernel
//! returns are newtypes, so an unknown value is never undefined behaviour.
//! The few exports that take a 64-bit value have adapters that join the
//! SysV register pair into one register.
//!
//! Imports are `unsafe extern "C" fn`. The few that take no address or
//! object the kernel uses and have no effect that safe code relies on are
//! declared `safe` and are plain `extern "C" fn`.

#![no_std]
#![allow(non_snake_case, non_upper_case_globals)]
// The crate documentation links console-only items, which other targets
// lack unless built with `--cfg xbox360_docs`.
#![cfg_attr(
    not(any(target_os = "xbox360", xbox360_docs)),
    allow(rustdoc::broken_intra_doc_links)
)]

/// Define a bit flag newtype over `$bits` with a constant for every named
/// bit, set operations, `|`, `&`, `!`, and a `Debug` form that lists the
/// named bits and prints the other bits in hex. `Default` is the empty set.
///
/// ```ignore
/// xbox360_sys::flags! {
///     /// Options of a request.
///     pub struct RequestFlags(u32) {
///         /// Wait for completion.
///         WAIT = 0x4,
///         SPIN = 0x10,
///     }
/// }
///
/// let flags = RequestFlags::WAIT | RequestFlags::from_bits_retain(0x100);
/// assert!(flags.contains(RequestFlags::WAIT));
/// ```
///
/// The type is `#[repr(transparent)]` over `$bits`, so it can be a field of
/// a `#[repr(C)]` structure or an argument of an import.
#[macro_export]
macro_rules! flags {
    (
        $(#[$meta:meta])*
        pub struct $name:ident($bits:ty) {
            $($(#[$flag_meta:meta])* $flag:ident = $value:expr,)*
        }
    ) => {
        $(#[$meta])*
        #[repr(transparent)]
        #[derive(
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::core::default::Default,
            ::core::cmp::PartialEq,
            ::core::cmp::Eq,
            ::core::hash::Hash,
        )]
        pub struct $name($bits);

        impl $name {
            $($(#[$flag_meta])* pub const $flag: Self = Self($value);)*

            /// Every named bit.
            const NAMED: $bits = 0 $(| $value)*;

            /// No flags.
            pub const fn empty() -> Self {
                Self(0)
            }

            /// Every named flag.
            pub const fn all() -> Self {
                Self(Self::NAMED)
            }

            pub const fn bits(self) -> $bits {
                self.0
            }

            /// `None` if `bits` has a bit without a name.
            pub const fn from_bits(bits: $bits) -> ::core::option::Option<Self> {
                if bits & !Self::NAMED == 0 {
                    ::core::option::Option::Some(Self(bits))
                } else {
                    ::core::option::Option::None
                }
            }

            /// `bits` including bits without a name, for values passed on
            /// unchanged.
            pub const fn from_bits_retain(bits: $bits) -> Self {
                Self(bits)
            }

            pub const fn union(self, other: Self) -> Self {
                Self(self.0 | other.0)
            }

            /// The flags of `self` that `other` does not have.
            pub const fn difference(self, other: Self) -> Self {
                Self(self.0 & !other.0)
            }

            pub const fn is_empty(self) -> bool {
                self.0 == 0
            }

            /// Whether every flag of `other` is set.
            pub const fn contains(self, other: Self) -> bool {
                self.0 & other.0 == other.0
            }

            /// Whether any flag of `other` is set.
            pub const fn intersects(self, other: Self) -> bool {
                self.0 & other.0 != 0
            }

            /// Set the flags of `other`.
            pub const fn insert(&mut self, other: Self) {
                self.0 |= other.0;
            }

            /// Clear the flags of `other`.
            pub const fn remove(&mut self, other: Self) {
                self.0 &= !other.0;
            }
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(::core::concat!(::core::stringify!($name), "("))?;
                // A type without named bits never reassigns these.
                #[allow(unused_mut)]
                let mut remaining = self.0;
                #[allow(unused_mut)]
                let mut separator = "";
                $(
                    if Self::$flag.0 != 0 && self.contains(Self::$flag) && remaining & Self::$flag.0 != 0 {
                        f.write_str(separator)?;
                        f.write_str(::core::stringify!($flag))?;
                        remaining &= !Self::$flag.0;
                        separator = " | ";
                    }
                )*
                if remaining != 0 || separator.is_empty() {
                    f.write_str(separator)?;
                    ::core::write!(f, "{:#x}", remaining)?;
                }
                f.write_str(")")
            }
        }

        impl ::core::ops::BitOr for $name {
            type Output = Self;

            fn bitor(self, other: Self) -> Self {
                Self(self.0 | other.0)
            }
        }

        impl ::core::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, other: Self) {
                self.0 |= other.0;
            }
        }

        impl ::core::ops::BitAnd for $name {
            type Output = Self;

            fn bitand(self, other: Self) -> Self {
                Self(self.0 & other.0)
            }
        }

        impl ::core::ops::BitAndAssign for $name {
            fn bitand_assign(&mut self, other: Self) {
                self.0 &= other.0;
            }
        }

        /// The named flags that `self` does not have.
        impl ::core::ops::Not for $name {
            type Output = Self;

            fn not(self) -> Self {
                Self(!self.0 & Self::NAMED)
            }
        }
    };
}

/// Define a fieldless enum with the given representation and values, with
/// `raw`, `from_raw` and `TryFrom` of the representation. `TryFrom` fails
/// with [`UnknownValue`] for a value without a variant.
///
/// ```ignore
/// xbox360_sys::value_enum! {
///     /// Why the loader calls a module entry point.
///     pub enum EntryReason: u32 {
///         Detach = 0,
///         Attach = 1,
///     }
/// }
///
/// assert_eq!(EntryReason::from_raw(1), Some(EntryReason::Attach));
/// ```
#[macro_export]
macro_rules! value_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident: $repr:ident {
            $($(#[$variant_meta:meta])* $variant:ident = $value:expr,)*
        }
    ) => {
        $(#[$meta])*
        #[repr($repr)]
        #[derive(
            ::core::fmt::Debug,
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::core::cmp::PartialEq,
            ::core::cmp::Eq,
            ::core::hash::Hash,
        )]
        pub enum $name {
            $($(#[$variant_meta])* $variant = $value,)*
        }

        impl $name {
            pub const fn raw(self) -> $repr {
                self as $repr
            }

            /// `None` for a value without a variant.
            pub const fn from_raw(value: $repr) -> ::core::option::Option<Self> {
                $(if value == $value {
                    return ::core::option::Option::Some(Self::$variant);
                })*
                ::core::option::Option::None
            }
        }

        impl ::core::convert::TryFrom<$repr> for $name {
            type Error = $crate::UnknownValue<$repr>;

            fn try_from(
                value: $repr,
            ) -> ::core::result::Result<Self, $crate::UnknownValue<$repr>> {
                Self::from_raw(value).ok_or($crate::UnknownValue { value })
            }
        }

        impl ::core::convert::From<$name> for $repr {
            fn from(value: $name) -> $repr {
                value as $repr
            }
        }
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

pub mod image;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub mod mmio;
pub mod status;
pub mod xam;
pub mod xboxkrnl;

pub mod processor;

#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub mod interlocked;

/// A value that no variant of the enum it was converted to has.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnknownValue<T> {
    pub value: T,
}

impl<T: core::fmt::Debug> core::fmt::Display for UnknownValue<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "value {:?} has no variant", self.value)
    }
}

impl<T: core::fmt::Debug> core::error::Error for UnknownValue<T> {}

/// Library index shared with `xex-link`'s `LIBRARIES`.
pub mod lib_index {
    pub const XBOXKRNL: u32 = 0;
    pub const XAM: u32 = 1;
    pub const XBDM: u32 = 2;
}

/// An export ordinal, as `XexGetProcedureAddress` takes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ordinal(core::num::NonZeroU16);

impl Ordinal {
    /// `None` for 0, which no export has.
    pub const fn new(ordinal: u16) -> Option<Self> {
        match core::num::NonZeroU16::new(ordinal) {
            Some(ordinal) => Some(Self(ordinal)),
            None => None,
        }
    }

    pub const fn get(self) -> u16 {
        self.0.get()
    }
}

/// Define an import: a stub with the signature callers use, and the thunk it
/// calls. Arguments pass through in r3-r10 to the resolved export.
/// Stack-passed arguments are not forwarded, so an import takes at most
/// eight arguments. [`import_thunk_stack!`] takes nine or ten.
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

/// A data import slot. Before any code of the module runs, the loader
/// replaces the import record in the slot with the address of the export,
/// a `T` that the kernel keeps for the life of the system. Safe code can
/// only read the slot.
#[repr(transparent)]
pub struct ImportedVariable<T: 'static> {
    address: core::sync::atomic::AtomicU32,
    _export: core::marker::PhantomData<&'static T>,
}

impl<T: Sync + 'static> ImportedVariable<T> {
    /// The export. `None` when the loader left the import unresolved:
    /// Xenia stores 0xDxxxBEEF in the slots of exports it does not provide,
    /// 0xF00DF00D for an ordinal it does not list, and 0xDEADC0DE for a
    /// function export imported as a variable.
    pub fn get(&self) -> Option<&'static T> {
        let address = resolved_address(self.address.load(core::sync::atomic::Ordering::Relaxed))?;
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
        let address = self.address.load(core::sync::atomic::Ordering::Relaxed);
        // SAFETY: the caller guarantees that the loader stored the address
        // of the export, a `T` that the kernel keeps for the life of the
        // system.
        unsafe { &*core::ptr::with_exposed_provenance::<T>(address as usize) }
    }
}

impl<T: 'static> core::fmt::Debug for ImportedVariable<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let address = self.address.load(core::sync::atomic::Ordering::Relaxed);
        write!(f, "ImportedVariable({address:#010x})")
    }
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

/// Call the function in r12 with r3-r10 unchanged and the full 64-bit
/// r14-r31 preserved. The CPU model of the target has no 64-bit mnemonics,
/// so `std`/`ld` are encoded directly.
///
/// # Safety
///
/// Enter it only with `b` from a stub that loads r12 with the target, as
/// [`bridged!`] does. It is not callable from Rust.
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

/// One `.xexports` record: big-endian ordinal, then the entry address.
/// xex-link turns the records into the XEX export table.
#[repr(C)]
pub struct ExportRecord {
    ordinal: u32,
    entry: *const (),
}

// SAFETY: the record is immutable and only read by the linker.
unsafe impl Sync for ExportRecord {}

impl ExportRecord {
    /// Used by [`export!`] and [`export_raw!`].
    #[doc(hidden)]
    pub const fn new(ordinal: u16, entry: *const ()) -> Self {
        assert!(ordinal != 0, "export ordinals start at 1");
        Self {
            ordinal: ordinal as u32,
            entry,
        }
    }
}

/// Export `$function` (as for [`bridged!`]) at `$ordinal` behind
/// [`entry_bridge`] or `bridge`. Ordinals that share one address are
/// declared once each.
#[macro_export]
macro_rules! export {
    ($ordinal:literal, $function:path, bridge = $bridge:path) => {
        const _: () = {
            $crate::bridged!(entry, $function, bridge = $bridge);

            #[used]
            #[unsafe(link_section = ".xexports")]
            static RECORD: $crate::ExportRecord =
                $crate::ExportRecord::new($ordinal, entry as *const ());
        };
    };
    ($ordinal:literal, $function:path) => {
        $crate::export!($ordinal, $function, bridge = $crate::entry_bridge);
    };
}

/// Export `$entry` at `$ordinal` with no bridge. `$entry` must implement the
/// console calling convention itself, including full 64-bit preservation of
/// any of r14-r31 it changes.
#[macro_export]
macro_rules! export_raw {
    ($ordinal:literal, $entry:path) => {
        const _: () = {
            #[used]
            #[unsafe(link_section = ".xexports")]
            static RECORD: $crate::ExportRecord =
                $crate::ExportRecord::new($ordinal, $entry as *const ());
        };
    };
}

#[cfg(test)]
mod tests {
    use crate::xam::UserIndex;
    use crate::xboxkrnl::AccessMask;
    use crate::xboxkrnl::CreateOptions;
    use crate::xboxkrnl::ThreadCreationFlags;

    #[test]
    fn flags_reject_unnamed_bits_unless_retained() {
        assert_eq!(
            CreateOptions::from_bits(0x60),
            Some(CreateOptions::SYNCHRONOUS_IO_NONALERT | CreateOptions::NON_DIRECTORY_FILE)
        );
        assert_eq!(CreateOptions::from_bits(0x8000_0000), None);
        assert_eq!(
            CreateOptions::from_bits_retain(0x8000_0020).bits(),
            0x8000_0020
        );
        let access = AccessMask::GENERIC_WRITE | AccessMask::SYNCHRONIZE;
        assert_eq!(access.bits(), 0x4010_0000);
        assert!(access.contains(AccessMask::SYNCHRONIZE));
        assert!(!access.contains(AccessMask::GENERIC_READ | AccessMask::SYNCHRONIZE));
        assert!(access.intersects(AccessMask::GENERIC_READ | AccessMask::SYNCHRONIZE));
    }

    #[test]
    fn flags_set_operations() {
        let mut access = AccessMask::READ_DATA | AccessMask::SYNCHRONIZE;
        access.insert(AccessMask::WRITE_DATA);
        access.remove(AccessMask::READ_DATA);
        assert_eq!(access, AccessMask::WRITE_DATA | AccessMask::SYNCHRONIZE);
        assert_eq!(
            access.difference(AccessMask::SYNCHRONIZE),
            AccessMask::WRITE_DATA
        );
        assert_eq!(!CreateOptions::all(), CreateOptions::empty());
        assert_eq!(
            (!AccessMask::empty()).bits(),
            AccessMask::all().bits()
        );
    }

    #[test]
    fn flags_debug_lists_named_bits_and_hex_rest() {
        extern crate std;
        use std::format;

        let access = AccessMask::from_bits_retain(0x4010_0000 | 0x800);
        assert_eq!(
            format!("{access:?}"),
            "AccessMask(SYNCHRONIZE | GENERIC_WRITE | 0x800)"
        );
        assert_eq!(format!("{:?}", AccessMask::empty()), "AccessMask(0x0)");
        assert_eq!(
            format!("{:?}", CreateOptions::DIRECTORY_FILE),
            "CreateOptions(DIRECTORY_FILE)"
        );
    }

    #[test]
    fn processor_mask_occupies_the_top_byte() {
        let flags = ThreadCreationFlags::processors(1 << 2) | ThreadCreationFlags::SUSPENDED;
        assert_eq!(flags.bits(), 0x0400_0001);
        assert_eq!(ThreadCreationFlags::processors(0xff).bits(), 0x3f00_0000);
        assert_eq!(ThreadCreationFlags::from_bits(flags.bits()), Some(flags));
    }

    #[test]
    fn irql_compares_the_low_byte() {
        use crate::xboxkrnl::Irql;
        assert_eq!(Irql::from_raw(0xdead_0002), Irql::DISPATCH);
        assert!(Irql::from_raw(0x0000_0101) < Irql::DISPATCH);
    }

    #[test]
    fn boolean8_tests_the_low_byte() {
        use crate::xboxkrnl::Boolean;
        use crate::xboxkrnl::Boolean8;
        assert!(!Boolean8(0xdead_be00).get());
        assert!(Boolean8(0x0000_0001).get());
        assert!(Boolean(0x0000_0100).get());
    }

    #[test]
    fn enums_convert_only_listed_values() {
        assert_eq!(UserIndex::from_raw(0xfe), Some(UserIndex::NoUser));
        assert_eq!(
            UserIndex::try_from(4),
            Err(crate::UnknownValue { value: 4 })
        );
        assert_eq!(u32::from(UserIndex::Any), 0xff);
    }

    #[test]
    fn unresolved_data_import_has_no_address() {
        assert_eq!(crate::resolved_address(0xd000_beef), None);
        assert_eq!(crate::resolved_address(0xd123_beef), None);
        assert_eq!(crate::resolved_address(0x801f_beef), Some(0x801f_beef));
        assert_eq!(crate::resolved_address(0xf00d_f00d), None);
        assert_eq!(crate::resolved_address(0xdead_c0de), None);
    }

    #[test]
    fn full_context_is_the_capture_request() {
        // RtlCaptureContext stores 0x17 in the flags word.
        assert_eq!(crate::xboxkrnl::ContextFlags::FULL.bits(), 0x17);
    }
}
