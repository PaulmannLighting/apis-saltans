use std::fmt::Display;
use std::ops::Deref;

use le_stream::{FromLeStream, Prefixed, ToLeStream};
use macaddr::MacAddr8;
use zigbee::Cluster;

use crate::Service;

/// Parent Announcement Service.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct ParentAnnce {
    child_info: Prefixed<u8, Box<[MacAddr8]>>,
}

impl ParentAnnce {
    /// Creates a new `ParentAnnce`.
    ///
    /// # Errors
    ///
    /// Returns the child info whose size could not be represented as `u8`.
    pub fn new(child_info: Box<[MacAddr8]>) -> Result<Self, Box<[MacAddr8]>> {
        child_info.try_into().map(|child_info| Self { child_info })
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
    type Error = Box<[MacAddr8]>;

    fn try_from(value: Box<[MacAddr8]>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<Vec<MacAddr8>> for ParentAnnce {
    type Error = Box<[MacAddr8]>;

    fn try_from(value: Vec<MacAddr8>) -> Result<Self, Self::Error> {
        Self::new(value.into_boxed_slice())
    }
}

impl TryFrom<&[MacAddr8]> for ParentAnnce {
    type Error = Box<[MacAddr8]>;

    fn try_from(value: &[MacAddr8]) -> Result<Self, Self::Error> {
        Self::new(value.into())
    }
}
