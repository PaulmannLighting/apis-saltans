use core::iter::Chain;

use le_stream::{FromLeStreamTagged, ToLeStream};
use repr_discriminant::ReprDiscriminant;
use zigbee::Parsable;

pub use self::name_support::NameSupport;

mod name_support;

/// Available attribute for the `Groups` cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[derive(ReprDiscriminant, FromLeStreamTagged)]
pub enum Attribute {
    /// Flag indicating whether the group name is supported by the device.
    NameSupport(Parsable<u8, NameSupport>) = 0x0000,
}

impl ToLeStream for Attribute {
    type Iter = Chain<<u16 as ToLeStream>::Iter, <NameSupport as ToLeStream>::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::NameSupport(name_support) => self
                .discriminant()
                .to_le_stream()
                .chain(name_support.to_le_stream()),
        }
    }
}
