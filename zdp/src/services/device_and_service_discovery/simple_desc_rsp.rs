use std::fmt::Display;

use le_stream::{FromLeStream, Prefixed, ToLeStream};
use zigbee::Cluster;

use crate::{Service, SimpleDescriptor, Status};

/// Simple Descriptor Response.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct SimpleDescRsp {
    status: u8,
    nwk_addr_of_interest: u16,
    descriptors: Prefixed<u8, Box<[SimpleDescriptor]>>,
}

impl SimpleDescRsp {
    /// Creates a new Simple Descriptor Response.
    #[must_use]
    pub fn new(
        status: Status,
        nwk_addr_of_interest: u16,
        descriptors: Prefixed<u8, Box<[SimpleDescriptor]>>,
    ) -> Self {
        Self {
            status: status.into(),
            nwk_addr_of_interest,
            descriptors,
        }
    }

    /// Creates a new Simple Descriptor Response.
    ///
    /// # Errors
    ///
    /// Returns an error if the provided slice cannot be converted into a prefixed boxed slice.
    pub fn try_new(
        status: Status,
        nwk_addr_of_interest: u16,
        descriptors: &[SimpleDescriptor],
    ) -> Result<Self, Box<[SimpleDescriptor]>> {
        Box::<[SimpleDescriptor]>::from(descriptors)
            .try_into()
            .map(|descriptors| Self::new(status, nwk_addr_of_interest, descriptors))
    }

    /// Return the status.
    ///
    /// # Errors
    ///
    /// Returns an error if the status code is not a valid `Status`.
    pub fn status(&self) -> Result<Status, u8> {
        self.status.try_into()
    }

    /// Return the network address of interest.
    #[must_use]
    pub const fn nwk_addr_of_interest(&self) -> u16 {
        self.nwk_addr_of_interest
    }

    /// Return the descriptors.
    #[must_use]
    pub fn descriptors(&self) -> &[SimpleDescriptor] {
        &self.descriptors
    }
}

impl Cluster for SimpleDescRsp {
    const ID: u16 = 0x8004;
}

impl Service for SimpleDescRsp {
    const NAME: &'static str = "Simple_Desc_rsp";
}

impl Display for SimpleDescRsp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {{ status: {:#04X}, nwk_addr_of_interest: {:#06X}, descriptors: {:?} }}",
            Self::NAME,
            self.status,
            self.nwk_addr_of_interest,
            self.descriptors
        )
    }
}
