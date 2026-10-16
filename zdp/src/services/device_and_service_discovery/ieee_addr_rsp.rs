pub use self::response::IeeeAddrRspResponse;
use super::address_response::AddressResponse;
use crate::Status;
#[cfg(not(feature = "alloc"))]
use crate::storage::Addresses;
use crate::storage::unpack;

mod response;

crate::zdp_command! {
    /// IEEE Address Response.
    IeeeAddrRsp => IEEE_addr_rsp;
    cluster_id: 0x8001;
    group: DeviceAndServiceDiscovery;
    fields {
        payload: AddressResponse,
    }
    constructor {
        /// Creates a new IEEE Address Response.
        #[must_use]
        pub fn new(response: Result<IeeeAddrRspResponse, Status>) -> Self {
            let payload = match response {
                Ok(IeeeAddrRspResponse::Single { ieee_addr_remote_dev, nwk_addr_remote_dev }) =>
                    AddressResponse::single(ieee_addr_remote_dev, nwk_addr_remote_dev),
                Ok(IeeeAddrRspResponse::Extended {
                    ieee_addr_remote_dev, nwk_addr_remote_dev, start_index, nwk_addr_assoc_dev_list,
                }) => AddressResponse::extended(
                    ieee_addr_remote_dev, nwk_addr_remote_dev, start_index, unpack(nwk_addr_assoc_dev_list),
                ),
                Err(status) => AddressResponse::failure(status),
            };
            Self { payload }
        }
    }
    getters {
        /// Return the status of the response.
        ///
        /// # Errors
        ///
        /// Returns the raw status code if the conversion to a [`Status`] fails.
        pub fn status(&self) -> Result<Status, u8> {
            self.payload.status()
        }
    }
    le_stream {
        from {
            fn from_le_stream<T>(bytes: T) -> Option<Self>
            where
                T: Iterator<Item = u8>,
            {
                AddressResponse::from_le_stream(bytes).map(|payload| Self { payload })
            }
        }
        to {
            #[cfg(feature = "alloc")]
            type Iter = alloc::vec::IntoIter<u8>;
            #[cfg(not(feature = "alloc"))]
            type Iter = crate::storage::Stream<<Addresses as IntoIterator>::IntoIter>;

            fn to_le_stream(self) -> Self::Iter {
                self.payload.to_le_stream()
            }
        }
    }
}
