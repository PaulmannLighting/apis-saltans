use std::fmt::Display;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::Cluster;

use crate::{ByteSizedVec, Command, Displayable, Service, Status};

/// Match Descriptor Response.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct MatchDescRsp {
    status: u8,
    nwk_addr_of_interest: u16,
    matches: ByteSizedVec<u8>,
}

impl MatchDescRsp {
    /// Creates a new `MatchDescRsp`.
    #[must_use]
    pub const fn new(status: Status, nwk_addr_of_interest: u16, matches: ByteSizedVec<u8>) -> Self {
        Self {
            status: status as u8,
            nwk_addr_of_interest,
            matches,
        }
    }

    /// Returns the status.
    ///
    /// # Errors
    ///
    /// Returns an error if the status code is invalid.
    pub fn status(&self) -> Result<Status, u8> {
        self.status.try_into()
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
            "{} {{ status: {}, nwk_addr_of_interest: {:#06X}, matches: [",
            Self::NAME,
            self.status().display(),
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

impl From<MatchDescRsp> for Command {
    fn from(value: MatchDescRsp) -> Self {
        Self::DeviceAndServiceDiscovery(value.into())
    }
}
