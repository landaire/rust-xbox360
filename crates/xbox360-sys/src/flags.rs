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
                    ::core::write!(f, "{remaining:#x}")?;
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

#[cfg(test)]
mod tests {
    use crate::xboxkrnl::AccessMask;
    use crate::xboxkrnl::CreateOptions;

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
        assert_eq!((!AccessMask::empty()).bits(), AccessMask::all().bits());
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
}
