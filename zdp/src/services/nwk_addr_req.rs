use le_stream::{FromLeStream, ToLeStream};
use macaddr::MacAddr8;
use num_traits::FromPrimitive;
use zigbee::Cluster;

pub use self::request_type::RequestType;
use crate::Service;

mod request_type;

/// Request parameters for network address request.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct NwkAddrReq {
    ieee_addr: MacAddr8,
    request_type: u8,
    start_index: u8,
}

impl NwkAddrReq {
    /// Creates a new `NwkAddrReq`.
    #[must_use]
    pub const fn new(ieee_addr: MacAddr8, request_type: RequestType, start_index: u8) -> Self {
        Self {
            ieee_addr,
            request_type: request_type as u8,
            start_index,
        }
    }

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

impl Cluster for NwkAddrReq {
    const ID: u16 = 0x0000;
}

impl Service for NwkAddrReq {
    const NAME: &'static str = "NWK_addr_req";
}
