use std::fmt::Display;

use le_stream::{FromLeStream, ToLeStream};
use apis_saltans_core::{Cluster, Endpoint};

use crate::services::DeviceAndServiceDiscovery;
use crate::{ByteSizedVec, Command, Service, Status};

/// Active Endpoint Response.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct ActiveEpRsp {
    status: u8,
    nwk_addr_of_interest: u16,
    active_eps: ByteSizedVec<Endpoint>,
}

impl ActiveEpRsp {
    /// Creates a new Active Endpoint Response.
    #[must_use]
    pub fn new(
        status: Status,
        nwk_addr_of_interest: u16,
        active_eps: ByteSizedVec<Endpoint>,
    ) -> Self {
        Self {
            status: status.into(),
            nwk_addr_of_interest,
            active_eps,
        }
    }

    /// Return the status of the response.
    ///
    /// # Errors
    ///
    /// Returns the raw status code if the conversion to a [`Status`] fails.
    pub fn status(&self) -> Result<Status, u8> {
        self.status.try_into()
    }

    /// Return the network address of interest.
    #[must_use]
    pub const fn nwk_addr_of_interest(&self) -> u16 {
        self.nwk_addr_of_interest
    }

    /// Return the active endpoints.
    #[must_use]
    pub fn active_eps(&self) -> &[Endpoint] {
        &self.active_eps
    }

    /// Return the active endpoints, consuming the [`ActiveEpRsp`].
    #[must_use]
    pub fn into_active_eps(self) -> ByteSizedVec<Endpoint> {
        self.active_eps
    }
}

impl Cluster for ActiveEpRsp {
    const ID: u16 = 0x8005;
}

impl Service for ActiveEpRsp {
    const NAME: &'static str = "Active_EP_rsp";
}

impl Display for ActiveEpRsp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {{ status: {:#04X}, nwk_addr_of_interest: {:#06X}, active_eps: {:#04X?} }}",
            Self::NAME,
            self.status,
            self.nwk_addr_of_interest,
            self.active_eps
        )
    }
}

impl IntoIterator for ActiveEpRsp {
    type Item = <ByteSizedVec<Endpoint> as IntoIterator>::Item;
    type IntoIter = <ByteSizedVec<Endpoint> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.active_eps.into_iter()
    }
}

impl TryFrom<Command> for ActiveEpRsp {
    type Error = Command;

    fn try_from(cmd: Command) -> Result<Self, Self::Error> {
        if let Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::ActiveEpRsp(
            active_eps,
        )) = cmd
        {
            Ok(*active_eps)
        } else {
            Err(cmd)
        }
    }
}
