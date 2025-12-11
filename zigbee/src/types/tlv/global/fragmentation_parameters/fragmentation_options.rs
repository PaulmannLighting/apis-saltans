use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

/// Fragmentation options.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, ToLeStream)]
#[repr(transparent)]
pub struct FragmentationOptions(u8);

bitflags! {
    impl FragmentationOptions: u8 {
        /// If this bit is set, fragmentation is supported.
        const FRAGMENTATION_SUPPORTED = 0b1000_0000;
    }
}

impl FromLeStream for FragmentationOptions {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        u8::from_le_stream(bytes).map(Self::from_bits_retain)
    }
}
