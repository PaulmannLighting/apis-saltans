use core::iter::Chain;

use le_stream::{FromLeStream, FromLeStreamTagged, ToLeStream};
use repr_discriminant::repr_discriminant;

/// Attributes for the Identify cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr_discriminant(u16, id)]
pub enum Attribute {
    /// Remaining length of time, in seconds, that the device will continue to identify itself.
    IdentifyTime(u16) = 0x0000,
}

impl FromLeStreamTagged for Attribute {
    type Tag = u16;

    fn from_le_stream_tagged<T>(tag: Self::Tag, bytes: T) -> Result<Option<Self>, Self::Tag>
    where
        T: Iterator<Item = u8>,
    {
        match tag {
            0x0000 => Ok(u16::from_le_stream(bytes).map(Self::IdentifyTime)),
            unknown => Err(unknown),
        }
    }
}

impl ToLeStream for Attribute {
    type Iter = Chain<<u16 as ToLeStream>::Iter, <u16 as ToLeStream>::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::IdentifyTime(time) => self.id().to_le_stream().chain(time.to_le_stream()),
        }
    }
}
