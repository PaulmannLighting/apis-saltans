use zb_core::Endpoint;
use zb_core::endpoint::Reserved;

use crate::SimpleDescRsp;

crate::zdp_command! {
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
        pub const fn new(nwk_address_of_interest: u16, endpoint: Endpoint) -> Self {
            Self {
                nwk_address_of_interest,
                endpoint: endpoint.as_u8(),
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
        ///
        /// # Errors
        ///
        /// Returns [`Reserved`] if the raw endpoint value is reserved.
        pub fn endpoint(self) -> Result<Endpoint, Reserved> {
            self.endpoint.try_into()
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
