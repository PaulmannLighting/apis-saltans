use std::ops::Deref;

use le_stream::{FromLeStream, Prefixed, ToLeStream};
use macaddr::MacAddr8;
use zigbee::Cluster;

use crate::Service;

/// Clear All Bindings Request
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct ClearAllBindingsReq {
    eui64s: Prefixed<u8, Box<[MacAddr8]>>,
}

impl ClearAllBindingsReq {
    /// Creates a new `ClearAllBindingsReq`.
    ///
    /// # Errors
    ///
    /// Returns the EUI64 list whose size could not be represented as `u8`.
    pub fn new(eui64s: Box<[MacAddr8]>) -> Result<Self, Box<[MacAddr8]>> {
        eui64s.try_into().map(|eui64s| Self { eui64s })
    }

    /// Returns a reference to the EUI64 list.
    #[must_use]
    pub fn eui64s(&self) -> &[MacAddr8] {
        &self.eui64s
    }
}

impl Cluster for ClearAllBindingsReq {
    const ID: u16 = 0x002b;
}

impl Service for ClearAllBindingsReq {
    const NAME: &'static str = "Clear_All_Bindings_req";
}

impl Deref for ClearAllBindingsReq {
    type Target = [MacAddr8];

    fn deref(&self) -> &Self::Target {
        &self.eui64s
    }
}

impl TryFrom<Box<[MacAddr8]>> for ClearAllBindingsReq {
    type Error = Box<[MacAddr8]>;

    fn try_from(value: Box<[MacAddr8]>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<Vec<MacAddr8>> for ClearAllBindingsReq {
    type Error = Box<[MacAddr8]>;

    fn try_from(value: Vec<MacAddr8>) -> Result<Self, Self::Error> {
        Self::new(value.into_boxed_slice())
    }
}

impl TryFrom<&[MacAddr8]> for ClearAllBindingsReq {
    type Error = Box<[MacAddr8]>;

    fn try_from(value: &[MacAddr8]) -> Result<Self, Self::Error> {
        Self::new(value.into())
    }
}
