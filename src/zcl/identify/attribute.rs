use core::iter::Chain;

use le_stream::ToLeStream;
use le_stream::derive::FromLeStreamTagged;
use repr_discriminant::repr_discriminant;

/// Attributes for the Identify cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr_discriminant(u16, id)]
#[derive(FromLeStreamTagged)]
pub enum Attribute {
    /// Remaining length of time, in seconds, that the device will continue to identify itself.
    IdentifyTime(u16) = 0x0000,
}

impl ToLeStream for Attribute {
    type Iter = Chain<<u16 as ToLeStream>::Iter, <u16 as ToLeStream>::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::IdentifyTime(time) => self.id().to_le_stream().chain(time.to_le_stream()),
        }
    }
}
