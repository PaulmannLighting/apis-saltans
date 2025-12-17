use std::fmt::Display;

use le_stream::{FromLeStream, Prefixed, ToLeStream};
use zigbee::Cluster;

use crate::Service;

/// Match Descriptor Response.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct MatchDescRsp {
    status: u8,
    nwk_addr_of_interest: u16,
    matches: Prefixed<u8, Box<[u8]>>,
}

impl MatchDescRsp {
    /// Creates a new `MatchDescRsp`.
    ///
    /// # Errors
    ///
    /// Returns an error if the length of `matches` exceeds `u8::MAX`.
    pub fn new(
        status: u8,
        nwk_addr_of_interest: u16,
        matches: Box<[u8]>,
    ) -> Result<Self, Box<[u8]>> {
        Ok(Self {
            status,
            nwk_addr_of_interest,
            matches: matches.try_into()?,
        })
    }

    /// Returns the status.
    #[must_use]
    pub const fn status(&self) -> u8 {
        self.status
    }

    /// Returns the network address of interest.
    #[must_use]
    pub const fn nwk_addr_of_interest(&self) -> u16 {
        self.nwk_addr_of_interest
    }

    /// Returns a reference to the list of matched endpoints.
    #[must_use]
    pub fn matches(&self) -> &[u8] {
        &self.matches
    }
}

impl Cluster for MatchDescRsp {
    const ID: u16 = 0x8006;
}

impl Service for MatchDescRsp {
    const NAME: &'static str = "Match_Desc_rsp";
}

impl Display for MatchDescRsp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {{ status: {:#04X}, nwk_addr_of_interest: {:#06X}, matches: [",
            Self::NAME,
            self.status,
            self.nwk_addr_of_interest,
        )?;

        let mut endpoints = self.matches.iter();

        if let Some(endpoint) = endpoints.next() {
            write!(f, "{endpoint:#04X}")?;

            for endpoint in endpoints {
                write!(f, ", {endpoint:#04X}")?;
            }
        }

        write!(f, "] }}")
    }
}
