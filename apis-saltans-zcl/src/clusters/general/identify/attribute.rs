use core::iter::Chain;

use apis_saltans_core::types::{Type, Uint16};
use le_stream::{FromLeStream, ToLeStream};
use repr_discriminant::ReprDiscriminant;

/// Attributes for the Identify cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[derive(ReprDiscriminant, FromLeStream)]
pub enum Attribute {
    /// Remaining length of time, in seconds, that the device will continue to identify itself.
    IdentifyTime(u16) = 0x0000,
}

impl ToLeStream for Attribute {
    type Iter = Chain<<u16 as ToLeStream>::Iter, <Type as ToLeStream>::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::IdentifyTime(time) => self
                .discriminant()
                .to_le_stream()
                .chain(Type::Uint16(Uint16::new(time)).to_le_stream()),
        }
    }
}

pub mod readable {
    //! Readable attributes of the Identify cluster.

    pub use super::Attribute;
}

pub mod writable {
    //! Writable attributes of the Identify cluster.

    pub use super::Attribute;
}
