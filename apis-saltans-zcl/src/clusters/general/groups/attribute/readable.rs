//! Readable attributes of the Groups cluster.

use core::iter::Chain;

use le_stream::ToLeStream;
use repr_discriminant::ReprDiscriminant;
use apis_saltans_core::types::Type;

use crate::general::groups::NameSupport;

/// Available attribute for the `Groups` cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum Attribute {
    /// Flag indicating whether the group name is supported by the device.
    NameSupport(NameSupport) = 0x0000,
}

impl ToLeStream for Attribute {
    type Iter = Chain<<u16 as ToLeStream>::Iter, <Type as ToLeStream>::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::NameSupport(name_support) => self
                .discriminant()
                .to_le_stream()
                .chain(Type::Map8(name_support.into()).to_le_stream()),
        }
    }
}
