use apis_saltans_core::Endpoint;

use crate::SimpleDescRsp;

crate::services::zdp_command! {
    /// Simple Descriptor Request structure.
    derive { Copy }
    SimpleDescReq => Simple_Desc_req;
    cluster_id: 0x0004;
    group: DeviceAndServiceDiscovery;
    response: SimpleDescRsp;
    fields {
        nwk_address_of_interest: u16,
        endpoint: u8,
    }
    constructor {
        /// Creates a new `SimpleDescReq`.
        #[must_use]
        pub fn new(nwk_address_of_interest: u16, endpoint: Endpoint) -> Self {
            Self {
                nwk_address_of_interest,
                endpoint: endpoint.into(),
            }
        }
    }
    getters {
        /// Returns the network address of interest.
        #[must_use]
        pub const fn nwk_address_of_interest(self) -> u16 {
            self.nwk_address_of_interest
        }

        /// Returns the endpoint.
        #[must_use]
        pub const fn endpoint(self) -> u8 {
            self.endpoint
        }
    }
    display {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{} {{ nwk_address_of_interest: {:#06X}, endpoint: {:#04X} }}",
                Self::NAME,
                self.nwk_address_of_interest,
                self.endpoint
            )
        }
    }
}
