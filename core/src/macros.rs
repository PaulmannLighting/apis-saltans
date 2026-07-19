macro_rules! impl_bitflags_display_and_from_str {
    ($ty:ty) => {
        impl core::fmt::Display for $ty {
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                bitflags::parser::to_writer(self, formatter)
            }
        }

        impl core::str::FromStr for $ty {
            type Err = bitflags::parser::ParseError;

            fn from_str(flags: &str) -> Result<Self, Self::Err> {
                bitflags::parser::from_str(flags)
            }
        }
    };
}

macro_rules! impl_display {
    ($ty:ty, |$value:ident, $formatter:ident| $body:block) => {
        impl core::fmt::Display for $ty {
            fn fmt(&self, $formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let $value = self;
                $body
            }
        }
    };
}

macro_rules! impl_fmt_via_value {
    ($ty:ty, $value_ty:ty, |$value:ident| $raw:expr) => {
        impl core::fmt::Display for $ty {
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let $value = self;
                let raw: $value_ty = $raw;
                <$value_ty as core::fmt::Display>::fmt(&raw, formatter)
            }
        }

        impl core::fmt::LowerHex for $ty {
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let $value = self;
                let raw: $value_ty = $raw;
                <$value_ty as core::fmt::LowerHex>::fmt(&raw, formatter)
            }
        }

        impl core::fmt::UpperHex for $ty {
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let $value = self;
                let raw: $value_ty = $raw;
                <$value_ty as core::fmt::UpperHex>::fmt(&raw, formatter)
            }
        }
    };
}

macro_rules! impl_hex_via_value {
    ($ty:ty, $value_ty:ty, |$value:ident| $raw:expr) => {
        impl core::fmt::LowerHex for $ty {
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let $value = self;
                let raw: $value_ty = $raw;
                <$value_ty as core::fmt::LowerHex>::fmt(&raw, formatter)
            }
        }

        impl core::fmt::UpperHex for $ty {
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let $value = self;
                let raw: $value_ty = $raw;
                <$value_ty as core::fmt::UpperHex>::fmt(&raw, formatter)
            }
        }
    };
}

macro_rules! impl_display_and_hex_via_value {
    (
        $ty:ty,
        $value_ty:ty,
        |$hex_value:ident| $raw:expr,
        |$display_value:ident, $formatter:ident| $body:block
    ) => {
        impl_display!($ty, |$display_value, $formatter| $body);
        impl_hex_via_value!($ty, $value_ty, |$hex_value| $raw);
    };
}

macro_rules! impl_fmt_pair {
    ($ty:ty, $left_ty:ty, $right_ty:ty, |$value:ident| ($left:expr, $right:expr), $separator:literal) => {
        impl_fmt_pair!(
            @one,
            $ty,
            core::fmt::Display,
            $left_ty,
            $right_ty,
            |$value| ($left, $right),
            $separator
        );
        impl_fmt_pair!(
            @one,
            $ty,
            core::fmt::LowerHex,
            $left_ty,
            $right_ty,
            |$value| ($left, $right),
            $separator
        );
        impl_fmt_pair!(
            @one,
            $ty,
            core::fmt::UpperHex,
            $left_ty,
            $right_ty,
            |$value| ($left, $right),
            $separator
        );
    };
    (@one, $ty:ty, $tr:path, $left_ty:ty, $right_ty:ty, |$value:ident| ($left:expr, $right:expr), $separator:literal) => {
        impl $tr for $ty {
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let $value = self;
                let left: $left_ty = $left;
                let right: $right_ty = $right;

                <$left_ty as $tr>::fmt(&left, formatter)?;
                formatter.write_str($separator)?;
                <$right_ty as $tr>::fmt(&right, formatter)
            }
        }
    };
}

macro_rules! impl_fmt_enum {
    ($ty:ty { $($variant:ident($inner_ty:ty) => $label:literal),+ $(,)? }) => {
        impl_fmt_enum!(@one, $ty, core::fmt::Display { $($variant($inner_ty) => $label),+ });
        impl_fmt_enum!(@one, $ty, core::fmt::LowerHex { $($variant($inner_ty) => $label),+ });
        impl_fmt_enum!(@one, $ty, core::fmt::UpperHex { $($variant($inner_ty) => $label),+ });
    };
    (@one, $ty:ty, $tr:path { $($variant:ident($inner_ty:ty) => $label:literal),+ }) => {
        impl $tr for $ty {
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    $(
                        Self::$variant(value) => {
                            formatter.write_str($label)?;
                            formatter.write_str("(")?;
                            <$inner_ty as $tr>::fmt(value, formatter)?;
                            formatter.write_str(")")
                        }
                    )+
                }
            }
        }
    };
}
