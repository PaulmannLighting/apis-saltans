use std::fmt::Display;
use std::ops::Deref;

use heapless::CapacityError;
use le_stream::{FromLeStream, ToLeStream};
use macaddr::MacAddr8;
use apis_saltans_core::Cluster;

use crate::{ByteSizedVec, Service};

/// Parent Announcement Service.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct ParentAnnce {
    child_info: ByteSizedVec<MacAddr8>,
}

impl ParentAnnce {
    /// Creates a new `ParentAnnce`.
    #[must_use]
    pub const fn new(child_info: ByteSizedVec<MacAddr8>) -> Self {
        Self { child_info }
    }

    /// Returns a reference to the child info.
    #[must_use]
    pub fn child_info(&self) -> &[MacAddr8] {
        &self.child_info
    }
}

impl Cluster for ParentAnnce {
    const ID: u16 = 0x001F;
}

impl Service for ParentAnnce {
    const NAME: &'static str = "Parent_annce";
}

impl Deref for ParentAnnce {
    type Target = [MacAddr8];

    fn deref(&self) -> &Self::Target {
        &self.child_info
    }
}

impl Display for ParentAnnce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {{ child_info: [", Self::NAME)?;

        let mut mac_addresses = self.child_info().iter();

        if let Some(mac_address) = mac_addresses.next() {
            write!(f, "{mac_address}")?;

            for mac_address in mac_addresses {
                write!(f, ", {mac_address}")?;
            }
        }

        write!(f, "] }}")
    }
}

impl TryFrom<Box<[MacAddr8]>> for ParentAnnce {
    type Error = CapacityError;

    fn try_from(value: Box<[MacAddr8]>) -> Result<Self, Self::Error> {
        Self::try_from(&*value)
    }
}

impl TryFrom<Vec<MacAddr8>> for ParentAnnce {
    type Error = CapacityError;

    fn try_from(value: Vec<MacAddr8>) -> Result<Self, Self::Error> {
        Self::try_from(value.into_boxed_slice())
    }
}

impl TryFrom<&[MacAddr8]> for ParentAnnce {
    type Error = CapacityError;

    fn try_from(value: &[MacAddr8]) -> Result<Self, Self::Error> {
        value.try_into().map(Self::new)
    }
}
