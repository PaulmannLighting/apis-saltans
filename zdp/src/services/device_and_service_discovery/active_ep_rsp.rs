use zb_core::endpoint::Reserved;
use zb_core::{ByteSizedVec, Endpoint};

use crate::Status;

crate::zdp_command! {
    /// Active Endpoint Response.
    ActiveEpRsp => Active_EP_rsp;
    cluster_id: 0x8005;
    group: DeviceAndServiceDiscovery;
    fields {
        status: u8,
        nwk_addr_of_interest: u16,
        active_eps: ByteSizedVec<u8>,
    }
    constructor {
        /// Creates a new Active Endpoint Response.
        #[must_use]
        pub fn new(
            nwk_addr_of_interest: u16,
            active_eps: Result<ByteSizedVec<Endpoint>, Status>,
        ) -> Self {
            match active_eps {
                Ok(active_eps) => Self {
                    status: Status::Success.into(),
                    nwk_addr_of_interest,
                    active_eps: active_eps.into_iter().map(Into::into).collect(),
                },
                Err(status) => Self {
                    status: status.into(),
                    nwk_addr_of_interest,
                    active_eps: ByteSizedVec::new(),
                },
            }
        }
    }
    getters {
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
        pub fn active_eps(&self) -> impl Iterator<Item = Result<Endpoint, Reserved>> + '_ {
            self.active_eps.iter().copied().map(TryInto::try_into)
        }

        /// Return the raw active endpoint IDs.
        #[must_use]
        pub fn active_ep_ids(&self) -> &[u8] {
            &self.active_eps
        }

        /// Return the active endpoints, consuming the [`ActiveEpRsp`].
        pub fn into_active_eps(self) -> impl Iterator<Item = Result<Endpoint, Reserved>> {
            self.active_eps.into_iter().map(TryInto::try_into)
        }

        /// Return the raw active endpoint IDs, consuming the [`ActiveEpRsp`].
        #[must_use]
        pub fn into_active_ep_ids(self) -> ByteSizedVec<u8> {
            self.active_eps
        }
    }
    display {
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
}
