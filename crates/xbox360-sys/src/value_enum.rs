/// Define a fieldless enum with the given representation and values, with
/// `raw`, `from_raw` and `TryFrom` of the representation. `TryFrom` fails
/// with [`UnknownValue`](crate::UnknownValue) for a value without a variant.
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

#[cfg(test)]
mod tests {
    use crate::UnknownValue;
    use crate::xam::UserIndex;

    #[test]
    fn enums_convert_only_listed_values() {
        assert_eq!(UserIndex::from_raw(0xfe), Some(UserIndex::NoUser));
        assert_eq!(UserIndex::try_from(4), Err(UnknownValue { value: 4 }));
        assert_eq!(u32::from(UserIndex::Any), 0xff);
    }
}
