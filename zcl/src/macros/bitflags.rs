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

pub(crate) use impl_bitflags_display_and_from_str;
