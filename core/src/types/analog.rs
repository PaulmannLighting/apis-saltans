//! Analog data types.

use intx::{I24, I40, I48, I56, U24, U40, U48, U56};

macro_rules! analog_integer {
    (
        $(#[$attr:meta])*
        signed manual_serde $name:ident($inner:ty, $non_value:expr $(, $intermediate:ty)?)
        $(alias [$($alias:ident),* $(,)?])?;
    ) => {
        analog_integer! {
            @define
            [manual_serde]
            [$($intermediate)?]
            []
            $(#[$attr])*
            $name($inner, $non_value)
            $(alias [$($alias),*])?
        }
    };
    (
        $(#[$attr:meta])*
        signed $name:ident($inner:ty, $non_value:expr $(, $intermediate:ty)?)
        $(alias [$($alias:ident),* $(,)?])?;
    ) => {
        analog_integer! {
            @define
            []
            [$($intermediate)?]
            []
            $(#[$attr])*
            $name($inner, $non_value)
            $(alias [$($alias),*])?
        }
    };
    (
        $(#[$attr:meta])*
        unsigned manual_serde $name:ident($inner:ty, $non_value:expr $(, $intermediate:ty)?)
        $(alias [$($alias:ident),* $(,)?])?;
    ) => {
        analog_integer! {
            @define
            [manual_serde]
            [$($intermediate)?]
            [
                /// The minimum valid value.
                pub const MIN: Self = Self(<$inner>::from_ne_bytes(
                    [0; core::mem::size_of::<$inner>()],
                ));

                /// The maximum valid value.
                pub const MAX: Self = {
                    let mut bytes = Self::NON_VALUE.to_ne_bytes();
                    if cfg!(target_endian = "big") {
                        bytes[core::mem::size_of::<$inner>() - 1] = bytes
                            [core::mem::size_of::<$inner>() - 1]
                            .checked_sub(1)
                            .expect("NON_VALUE is not zero");
                    } else {
                        bytes[0] = bytes[0]
                            .checked_sub(1)
                            .expect("NON_VALUE is not zero");
                    }
                    Self(<$inner>::from_ne_bytes(bytes))
                };
            ]
            $(#[$attr])*
            $name($inner, $non_value)
            $(alias [$($alias),*])?
        }
    };
    (
        $(#[$attr:meta])*
        unsigned $name:ident($inner:ty, $non_value:expr $(, $intermediate:ty)?)
        $(alias [$($alias:ident),* $(,)?])?;
    ) => {
        analog_integer! {
            @define
            []
            [$($intermediate)?]
            [
                /// The minimum valid value.
                pub const MIN: Self = Self(<$inner>::from_ne_bytes(
                    [0; core::mem::size_of::<$inner>()],
                ));

                /// The maximum valid value.
                pub const MAX: Self = {
                    let mut bytes = Self::NON_VALUE.to_ne_bytes();
                    if cfg!(target_endian = "big") {
                        bytes[core::mem::size_of::<$inner>() - 1] = bytes
                            [core::mem::size_of::<$inner>() - 1]
                            .checked_sub(1)
                            .expect("NON_VALUE is not zero");
                    } else {
                        bytes[0] = bytes[0]
                            .checked_sub(1)
                            .expect("NON_VALUE is not zero");
                    }
                    Self(<$inner>::from_ne_bytes(bytes))
                };
            ]
            $(#[$attr])*
            $name($inner, $non_value)
            $(alias [$($alias),*])?
        }
    };
    (
        @define
        []
        []
        $unsigned_const:tt
        $(#[$attr:meta])*
        $name:ident($inner:ty, $non_value:expr)
        $(alias [$($alias:ident),*])?
    ) => {
        analog_integer! {
            @define_with_attrs
            [cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            []
            $unsigned_const
            $(#[$attr])*
            $name($inner, $non_value)
            $(alias [$($alias),*])?
        }
    };
    (
        @define
        []
        [$intermediate:ty]
        $unsigned_const:tt
        $(#[$attr:meta])*
        $name:ident($inner:ty, $non_value:expr)
        $(alias [$($alias:ident),*])?
    ) => {
        analog_integer! {
            @define_with_attrs
            []
            [$intermediate]
            $unsigned_const
            $(#[$attr])*
            $name($inner, $non_value)
            $(alias [$($alias),*])?
        }
    };
    (
        @define
        [manual_serde]
        $intermediate:tt
        $unsigned_const:tt
        $(#[$attr:meta])*
        $name:ident($inner:ty, $non_value:expr)
        $(alias [$($alias:ident),*])?
    ) => {
        analog_integer! {
            @define_with_attrs
            []
            $intermediate
            $unsigned_const
            $(#[$attr])*
            $name($inner, $non_value)
            $(alias [$($alias),*])?
        }
    };
    (
        @define_with_attrs
        [$($serde_attr:meta)?]
        [$($intermediate:ty)?]
        [$($unsigned_const:item)*]
        $(#[$attr:meta])*
        $name:ident($inner:ty, $non_value:expr)
        $(alias [$($alias:ident),*])?
    ) => {
        $(#[$attr])*
        $(#[$serde_attr])?
        #[derive(
            Clone,
            Copy,
            Debug,
            Default,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            le_stream::FromLeStream,
            le_stream::ToLeStream,
        )]
        #[repr(transparent)]
        pub struct $name($inner);

        impl $name {
            /// The inner non-value.
            const NON_VALUE: $inner = $non_value;

            $($unsigned_const)*

            /// The non-value.
            pub const NONE: Self = Self(Self::NON_VALUE);

            /// Create a new value from a raw inner value.
            #[must_use]
            pub const fn new(raw: $inner) -> Self {
                Self(raw)
            }

            /// Return the inner raw value.
            #[must_use]
            pub const fn into_inner(self) -> $inner {
                self.0
            }

            /// Return the inner raw value when it is not the non-value.
            #[must_use]
            pub const fn as_option(self) -> Option<$inner> {
                let value = self.0.to_ne_bytes();
                let non_value = Self::NON_VALUE.to_ne_bytes();
                let mut index = 0;

                while index < core::mem::size_of::<$inner>() {
                    if value[index] != non_value[index] {
                        return Some(self.0);
                    }
                    index += 1;
                }

                None
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                <$inner as core::fmt::Display>::fmt(&self.0, formatter)
            }
        }

        impl core::fmt::LowerHex for $name {
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                <$inner as core::fmt::LowerHex>::fmt(&self.0, formatter)
            }
        }

        impl core::fmt::UpperHex for $name {
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                <$inner as core::fmt::UpperHex>::fmt(&self.0, formatter)
            }
        }

        impl From<$name> for Option<$inner> {
            fn from(value: $name) -> Self {
                value.as_option()
            }
        }

        impl From<$name> for crate::types::Type {
            fn from(value: $name) -> Self {
                Self::$name(value)
            }
        }

        impl TryFrom<$name> for $inner {
            type Error = $name;

            fn try_from(value: $name) -> Result<Self, Self::Error> {
                value.as_option().ok_or(value)
            }
        }

        impl TryFrom<$inner> for $name {
            type Error = ();

            fn try_from(value: $inner) -> Result<Self, Self::Error> {
                if value == Self::NON_VALUE {
                    Err(())
                } else {
                    Ok(Self(value))
                }
            }
        }

        impl TryFrom<Option<$inner>> for $name {
            type Error = ();

            fn try_from(value: Option<$inner>) -> Result<Self, Self::Error> {
                value.map_or(Ok(Self::NONE), Self::try_from)
            }
        }

        impl TryFrom<crate::types::Type> for $name {
            type Error = crate::types::Type;

            fn try_from(value: crate::types::Type) -> Result<Self, Self::Error> {
                match value {
                    crate::types::Type::$name(value)
                    $(| $(crate::types::Type::$alias(value))|*)? => Ok(value),
                    other => Err(other),
                }
            }
        }

        analog_integer! {
            @intermediate_impl
            $name
            $inner
            [$($intermediate)?]
        }
    };
    (
        @intermediate_impl
        $name:ident
        $inner:ty
        []
    ) => {};
    (
        @intermediate_impl
        $name:ident
        $inner:ty
        [$intermediate:ty]
    ) => {
        impl From<$name> for Option<$intermediate> {
            fn from(value: $name) -> Self {
                Option::<$inner>::from(value).map(Into::into)
            }
        }

        impl TryFrom<$intermediate> for $name {
            type Error = Option<$intermediate>;

            fn try_from(value: $intermediate) -> Result<Self, Self::Error> {
                <$inner>::try_from(value).map_or(Err(Some(value)), |inner| {
                    Self::try_from(inner).map_err(|()| None)
                })
            }
        }

        #[cfg(feature = "serde")]
        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let value: $intermediate = self.into_inner().into();
                serde::Serialize::serialize(&value, serializer)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = <$intermediate as serde::Deserialize>::deserialize(deserializer)?;
                let inner = <$inner>::try_from(value).map_err(|_| {
                    serde::de::Error::custom(concat!(
                        "value is out of range for ",
                        stringify!($name)
                    ))
                })?;

                Ok(Self(inner))
            }
        }
    };
}

macro_rules! analog_integers {
    (
        signed { $($signed:tt)* }
        unsigned { $($unsigned:tt)* }
    ) => {
        analog_integers! { @signed $($signed)* }
        analog_integers! { @unsigned $($unsigned)* }
    };
    (@signed) => {};
    (
        @signed
        $(#[$attr:meta])*
        manual_serde $name:ident($inner:ty, $non_value:expr $(, $intermediate:ty)?)
        $(alias [$($alias:ident),* $(,)?])?;
        $($rest:tt)*
    ) => {
        analog_integer! {
            $(#[$attr])*
            signed manual_serde $name($inner, $non_value $(, $intermediate)?)
            $(alias [$($alias),*])?;
        }
        analog_integers! { @signed $($rest)* }
    };
    (
        @signed
        $(#[$attr:meta])*
        $name:ident($inner:ty, $non_value:expr $(, $intermediate:ty)?)
        $(alias [$($alias:ident),* $(,)?])?;
        $($rest:tt)*
    ) => {
        analog_integer! {
            $(#[$attr])*
            signed $name($inner, $non_value $(, $intermediate)?)
            $(alias [$($alias),*])?;
        }
        analog_integers! { @signed $($rest)* }
    };
    (@unsigned) => {};
    (
        @unsigned
        $(#[$attr:meta])*
        manual_serde $name:ident($inner:ty, $non_value:expr $(, $intermediate:ty)?)
        $(alias [$($alias:ident),* $(,)?])?;
        $($rest:tt)*
    ) => {
        analog_integer! {
            $(#[$attr])*
            unsigned manual_serde $name($inner, $non_value $(, $intermediate)?)
            $(alias [$($alias),*])?;
        }
        analog_integers! { @unsigned $($rest)* }
    };
    (
        @unsigned
        $(#[$attr:meta])*
        $name:ident($inner:ty, $non_value:expr $(, $intermediate:ty)?)
        $(alias [$($alias:ident),* $(,)?])?;
        $($rest:tt)*
    ) => {
        analog_integer! {
            $(#[$attr])*
            unsigned $name($inner, $non_value $(, $intermediate)?)
            $(alias [$($alias),*])?;
        }
        analog_integers! { @unsigned $($rest)* }
    };
}

analog_integers! {
    signed {
        /// The `8-bit signed integer` type, short `int8`.
        Int8(i8, 0x80_u8.cast_signed());

        /// The `16-bit signed integer` type, short `int16`.
        Int16(i16, 0x8000_u16.cast_signed());

        /// The `24-bit signed integer` type, short `int24`.
        Int24(
            I24,
            if cfg!(target_endian = "big") {
                I24::from_ne_bytes([0x80, 0x00, 0x00])
            } else {
                I24::from_ne_bytes([0x00, 0x00, 0x80])
            },
            i32
        );

        /// The `32-bit signed integer` type, short `int32`.
        Int32(i32, 0x8000_0000_u32.cast_signed());

        /// The `40-bit signed integer` type, short `int40`.
        Int40(
            I40,
            if cfg!(target_endian = "big") {
                I40::from_ne_bytes([0x80, 0x00, 0x00, 0x00, 0x00])
            } else {
                I40::from_ne_bytes([0x00, 0x00, 0x00, 0x00, 0x80])
            },
            i64
        );

        /// The `48-bit signed integer` type, short `int48`.
        Int48(
            I48,
            if cfg!(target_endian = "big") {
                I48::from_ne_bytes([0x80, 0x00, 0x00, 0x00, 0x00, 0x00])
            } else {
                I48::from_ne_bytes([0x00, 0x00, 0x00, 0x00, 0x00, 0x80])
            },
            i64
        );

        /// The `56-bit signed integer` type, short `int56`.
        Int56(
            I56,
            if cfg!(target_endian = "big") {
                I56::from_ne_bytes([0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
            } else {
                I56::from_ne_bytes([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80])
            },
            i64
        );

        /// The `64-bit signed integer` type, short `int64`.
        Int64(i64, 0x8000_0000_0000_0000_u64.cast_signed());
    }
    unsigned {
        /// The `8-bit unsigned integer` type, short `uint8`.
        Uint8(u8, 0xff);

        /// The `16-bit unsigned integer` type, short `uint16`.
        Uint16(u16, 0xffff);

        /// The `24-bit unsigned integer` type, short `uint24`.
        Uint24(U24, U24::from_ne_bytes([0xff, 0xff, 0xff]), u32);

        /// The `32-bit unsigned integer` type, short `uint32`.
        Uint32(u32, 0xffff);

        /// The `40-bit unsigned integer` type, short `uint40`.
        Uint40(U40, U40::from_ne_bytes([0xff, 0xff, 0xff, 0xff, 0xff]), u64);

        /// The `48-bit unsigned integer` type, short `uint48`.
        Uint48(
            U48,
            U48::from_ne_bytes([0xff, 0xff, 0xff, 0xff, 0xff, 0xff]),
            u64
        );

        /// The `56-bit unsigned integer` type, short `uint56`.
        Uint56(
            U56,
            U56::from_ne_bytes([0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]),
            u64
        );

        /// The `64-bit unsigned integer` type, short `uint64`.
        Uint64(u64, 0xffff_ffff_ffff_ffff);
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::format;
    use core::fmt::{Display, LowerHex, UpperHex};

    use intx::U24;

    use super::{Int16, Uint24};

    const SIGNED_INNER: i16 = -42;
    const UNSIGNED_INNER_BYTES: [u8; 3] = [0x12, 0x34, 0x56];

    fn assert_delegated_formatting<Wrapper, Inner>(wrapper: &Wrapper, inner: &Inner)
    where
        Wrapper: Display + LowerHex + UpperHex,
        Inner: Display + LowerHex + UpperHex,
    {
        assert_eq!(format!("{wrapper:+08}"), format!("{inner:+08}"));
        assert_eq!(format!("{wrapper:#010x}"), format!("{inner:#010x}"));
        assert_eq!(format!("{wrapper:#010X}"), format!("{inner:#010X}"));
    }

    #[test]
    fn aligned_integer_formatting_delegates_to_inner_type() {
        let wrapper = Int16::new(SIGNED_INNER);

        assert_delegated_formatting(&wrapper, &SIGNED_INNER);
    }

    #[test]
    fn unaligned_integer_formatting_delegates_to_inner_type() {
        let inner = U24::from_ne_bytes(UNSIGNED_INNER_BYTES);
        let wrapper = Uint24::new(inner);

        assert_delegated_formatting(&wrapper, &inner);
    }
}
