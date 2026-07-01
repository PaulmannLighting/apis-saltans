use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

/// A bitmask representing various configuration options.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, ToLeStream)]
#[repr(transparent)]
pub struct ConfigurationBitmask(u8);

bitflags! {
    impl ConfigurationBitmask: u8 {
        /// If this bit is set, indicates that an enhanced scan instead of an active scan should be performed.
        const ENHANCED = 0b100_0000;
    }
}

impl FromLeStream for ConfigurationBitmask {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        u8::from_le_stream(bytes).map(Self::from_bits_truncate)
    }
}
