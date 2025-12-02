use core::iter::Chain;

use le_stream::{FromLeStreamTagged, ToLeStream};
use repr_discriminant::ReprDiscriminant;

/// Attributes for the Identify cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[derive(ReprDiscriminant, FromLeStreamTagged)]
pub enum Attribute {
    /// Remaining length of time, in seconds, that the device will continue to identify itself.
    IdentifyTime(u16) = 0x0000,
}

impl ToLeStream for Attribute {
    type Iter = Chain<<u16 as ToLeStream>::Iter, <u16 as ToLeStream>::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::IdentifyTime(time) => self
                .discriminant()
                .to_le_stream()
                .chain(time.to_le_stream()),
        }
    }
}
