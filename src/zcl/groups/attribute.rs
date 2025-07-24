use core::iter::Chain;

use le_stream::ToLeStream;
use le_stream::derive::FromLeStreamTagged;
pub use name_support::NameSupport;
use repr_discriminant::ReprDiscriminant;

mod name_support;

/// Available attribute for the `Groups` cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[derive(ReprDiscriminant, FromLeStreamTagged)]
pub enum Attribute {
    /// Flag indicating whether the group name is supported by the device.
    NameSupport(NameSupport) = 0x0000,
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
