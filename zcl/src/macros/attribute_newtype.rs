macro_rules! zcl_attribute_newtype {
    (
        $(#[$attr:meta])*
        $vis:vis ranged struct $name:ident($inner:ty) = $min:literal..=$max:literal;
    ) => {
        $(#[$attr])*
        #[cfg_attr(
            feature = "serde",
            derive(serde::Serialize, serde::Deserialize),
            serde(transparent)
        )]
        #[derive(
            Clone,
            Copy,
            Debug,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            le_stream::ToLeStream,
        )]
        #[repr(transparent)]
        $vis struct $name($inner);

        impl $name {
            /// Minimum allowed value.
            pub const MIN: $inner = $min;

            /// Maximum allowed value.
            pub const MAX: $inner = $max;

            /// Try to create a new attribute value.
            #[must_use]
            pub fn try_new(value: $inner) -> Option<Self> {
                if (Self::MIN..=Self::MAX).contains(&value) {
                    Some(Self(value))
                } else {
                    None
                }
            }

            /// Return the inner value.
            #[must_use]
            pub const fn into_inner(self) -> $inner {
                self.0
            }
        }

        impl From<$name> for $inner {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl TryFrom<$inner> for $name {
            type Error = $inner;

            fn try_from(value: $inner) -> Result<Self, Self::Error> {
                Self::try_new(value).ok_or(value)
            }
        }

        impl le_stream::FromLeStream for $name {
            fn from_le_stream<T>(mut bytes: T) -> Option<Self>
            where
                T: Iterator<Item = u8>,
            {
                <$inner as le_stream::FromLeStream>::from_le_stream(&mut bytes)
                    .and_then(|value| value.try_into().ok())
            }
        }
    };
    (
        $(#[$attr:meta])*
        $vis:vis bitflags $name:ident($inner:ty) => $variant:ident {
            $(
                $(#[$($flag_attr:tt)*])*
                const $flag:ident = $value:expr;
            )+
        }
    ) => {
        $(#[$attr])*
        #[cfg_attr(
            feature = "serde",
            derive(serde::Serialize, serde::Deserialize),
            serde(transparent)
        )]
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
        $vis struct $name($inner);

        bitflags::bitflags! {
            impl $name: $inner {
                $(
                    $(#[$($flag_attr)*])*
                    const $flag = $value;
                )+
            }
        }

        $crate::macros::impl_bitflags_display_and_from_str!($name);

        $crate::macros::zcl_attribute_newtype! {
            @type_id_impl
            [$name]
            [$inner]
            [$variant]
        }

        impl From<$name> for zb_core::types::Type {
            fn from(value: $name) -> Self {
                Self::$variant(value.bits().into())
            }
        }

        impl TryFrom<zb_core::types::Type> for $name {
            type Error = zb_core::types::Type;

            fn try_from(value: zb_core::types::Type) -> Result<Self, Self::Error> {
                if let zb_core::types::Type::$variant(value) = value {
                    Ok(Self::from_bits_retain(value.into()))
                } else {
                    Err(value)
                }
            }
        }
    };
    (
        $(#[$attr:meta])*
        $vis:vis enum $name:ident: Enum8 {
            $(
                $(#[$variant_attr:meta])*
                $variant:ident = $value:expr,
            )+
        }
    ) => {
        $crate::macros::zcl_attribute_newtype! {
            @enum
            [$(#[$attr])*]
            [$vis]
            [$name]
            [Enum8]
            [
                $(
                    $(#[$variant_attr])*
                    $variant = $value,
                )+
            ]
        }
    };
    (
        $(#[$attr:meta])*
        $vis:vis enum $name:ident: Map8 {
            $(
                $(#[$variant_attr:meta])*
                $variant:ident = $value:expr,
            )+
        }
    ) => {
        $crate::macros::zcl_attribute_newtype! {
            @enum
            [$(#[$attr])*]
            [$vis]
            [$name]
            [Map8]
            [
                $(
                    $(#[$variant_attr])*
                    $variant = $value,
                )+
            ]
        }
    };
    (
        @enum
        [$($attr:tt)*]
        [$vis:vis]
        [$name:ident]
        [$type_variant:ident]
        [
            $(
                $(#[$variant_attr:meta])*
                $variant:ident = $value:expr,
            )+
        ]
    ) => {
        $($attr)*
        #[allow(clippy::enum_variant_names)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(
            Clone,
            Copy,
            Debug,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            num_enum::IntoPrimitive,
            num_enum::TryFromPrimitive,
        )]
        #[num_enum(error_type(name = u8, constructor = core::convert::identity))]
        #[repr(u8)]
        $vis enum $name {
            $(
                $(#[$variant_attr])*
                $variant = $value,
            )+
        }

        impl From<$name> for zb_core::types::Uint8 {
            fn from(value: $name) -> Self {
                Self::new(value.into())
            }
        }

        $crate::macros::zcl_attribute_newtype! {
            @enum_from_type
            [$name]
            [$type_variant]
        }


        $crate::macros::zcl_attribute_newtype! {
            @type_id_impl
            [$name]
            [u8]
            [$type_variant]
        }

        impl TryFrom<zb_core::types::Uint8> for $name {
            type Error = zb_core::types::Uint8;

            fn try_from(value: zb_core::types::Uint8) -> Result<Self, Self::Error> {
                Self::try_from(value.into_inner()).map_err(|_| value)
            }
        }

        $crate::macros::zcl_attribute_newtype! {
            @enum_try_from_type
            [$name]
            [$type_variant]
        }

        impl le_stream::FromLeStream for $name {
            fn from_le_stream<T>(mut bytes: T) -> Option<Self>
            where
                T: Iterator<Item = u8>,
            {
                u8::from_le_stream(&mut bytes).and_then(|value| Self::try_from(value).ok())
            }
        }

        impl le_stream::ToLeStream for $name {
            type Iter = <u8 as le_stream::ToLeStream>::Iter;

            fn to_le_stream(self) -> Self::Iter {
                u8::from(self).to_le_stream()
            }
        }
    };
    (@enum_from_type [$name:ident] [Enum8]) => {
        impl From<$name> for zb_core::types::Type {
            fn from(value: $name) -> Self {
                Self::Enum8(zb_core::types::Enum8::new(value.into()))
            }
        }
    };
    (@enum_from_type [$name:ident] [Map8]) => {
        impl From<$name> for zb_core::types::Type {
            fn from(value: $name) -> Self {
                Self::Map8(value.into())
            }
        }
    };
    (@enum_try_from_type [$name:ident] [Enum8]) => {
        impl TryFrom<zb_core::types::Type> for $name {
            type Error = zb_core::types::Type;

            fn try_from(value: zb_core::types::Type) -> Result<Self, Self::Error> {
                if let zb_core::types::Type::Enum8(value) = value {
                    Self::try_from(value.into_inner())
                        .map_err(|_| zb_core::types::Type::Enum8(value))
                } else {
                    Err(value)
                }
            }
        }
    };
    (@enum_try_from_type [$name:ident] [Map8]) => {
        impl TryFrom<zb_core::types::Type> for $name {
            type Error = zb_core::types::Type;

            fn try_from(value: zb_core::types::Type) -> Result<Self, Self::Error> {
                if let zb_core::types::Type::Map8(raw) = value {
                    raw.try_into().map_err(|_| zb_core::types::Type::Map8(raw))
                } else {
                    Err(value)
                }
            }
        }
    };
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident($inner:ty) => $variant:ident;
    ) => {
        $(#[$attr])*
        #[cfg_attr(
            feature = "serde",
            derive(serde::Serialize, serde::Deserialize),
            serde(transparent)
        )]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[repr(transparent)]
        $vis struct $name($inner);

        impl $name {
            /// Create a new attribute value.
            #[must_use]
            pub const fn new(value: $inner) -> Self {
                Self(value)
            }

            /// Return the inner value.
            #[must_use]
            pub const fn into_inner(self) -> $inner {
                self.0
            }
        }

        impl From<$inner> for $name {
            fn from(value: $inner) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $inner {
            fn from(value: $name) -> Self {
                value.0
            }
        }


        $crate::macros::zcl_attribute_newtype! {
            @type_id_impl
            [$name]
            [$inner]
            [$variant]
        }

        impl From<$name> for zb_core::types::Type {
            fn from(value: $name) -> Self {
                Self::$variant(value.0.into())
            }
        }

        impl TryFrom<zb_core::types::Type> for $name {
            type Error = zb_core::types::Type;

            fn try_from(value: zb_core::types::Type) -> Result<Self, Self::Error> {
                if let zb_core::types::Type::$variant(value) = value {
                    Ok(Self(value.into()))
                } else {
                    Err(value)
                }
            }
        }
    };
    (@type_id_impl [$name:ident] [$inner:ty] [Enum8]) => {
        impl zb_core::TypeId for $name {
            const ID: u8 = <zb_core::types::Enum8 as zb_core::TypeId>::ID;
        }
    };
    (@type_id_impl [$name:ident] [$inner:ty] [Map8]) => {
        impl zb_core::TypeId for $name {
            const ID: u8 = <u8 as zb_core::TypeId>::ID;
        }
    };
    (@type_id_impl [$name:ident] [$inner:ty] [Map16]) => {
        impl zb_core::TypeId for $name {
            const ID: u8 = <zb_core::types::Map16 as zb_core::TypeId>::ID;
        }
    };
    (@type_id_impl [$name:ident] [$inner:ty] [Map32]) => {
        impl zb_core::TypeId for $name {
            const ID: u8 = <zb_core::types::Map32 as zb_core::TypeId>::ID;
        }
    };
    (@type_id_impl [$name:ident] [$inner:ty] [$variant:ident]) => {
        impl zb_core::TypeId for $name {
            const ID: u8 = <$inner as zb_core::TypeId>::ID;
        }
    };
}

#[allow(unused_imports)]
pub(crate) use zcl_attribute_newtype;
