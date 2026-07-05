use apis_saltans_core::Endpoint;

use crate::{ByteSizedVec, Status};

crate::zdp_command! {
    /// Active Endpoint Response.
    ActiveEpRsp => Active_EP_rsp;
    cluster_id: 0x8005;
    group: DeviceAndServiceDiscovery;
    fields {
        status: u8,
        nwk_addr_of_interest: u16,
        active_eps: ByteSizedVec<Endpoint>,
    }
    constructor {
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
