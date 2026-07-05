use macaddr::MacAddr8;
use num_traits::FromPrimitive;

pub use self::request_type::RequestType;

mod request_type;

crate::zdp_command! {
    /// Request parameters for network address request.
    NwkAddrReq => NWK_addr_req;
    cluster_id: 0x0000;
    group: DeviceAndServiceDiscovery;
    fields {
        ieee_addr: MacAddr8,
        request_type: u8,
        start_index: u8,
    }
    constructor {
        /// Creates a new `NwkAddrReq`.
        #[must_use]
        pub const fn new(ieee_addr: MacAddr8, request_type: RequestType, start_index: u8) -> Self {
            Self {
                ieee_addr,
                request_type: request_type as u8,
                start_index,
            }
        }
    }
    getters {
        /// Returns the IEEE address.
        #[must_use]
        pub const fn ieee_addr(&self) -> MacAddr8 {
            self.ieee_addr
        }

        /// Returns the request type.
        ///
        /// # Errors
        ///
        /// Returns the raw request type byte if it does not correspond to a known [`RequestType`].
        pub fn request_type(&self) -> Result<RequestType, u8> {
            RequestType::from_u8(self.request_type).ok_or(self.request_type)
        }

        /// Returns the start index.
        #[must_use]
        pub const fn start_index(&self) -> u8 {
            self.start_index
        }
    }
    display {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{} {{ ieee_addr: {}, request_type: {:#04X}, start_index: {:#04X} }}",
                Self::NAME,
                self.ieee_addr,
                self.request_type,
                self.start_index
            )
        }
    }
}
