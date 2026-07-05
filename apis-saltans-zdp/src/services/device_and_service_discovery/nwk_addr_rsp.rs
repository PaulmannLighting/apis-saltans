use macaddr::MacAddr8;

pub use self::response::NwkAddrRspResponse;
use crate::Status;

mod response;

crate::zdp_command! {
    /// Network Address Response.
    NwkAddrRsp => NWK_addr_rsp;
    cluster_id: 0x8000;
    group: DeviceAndServiceDiscovery;
    fields {
        status: u8,
        ieee_addr_remote_dev: Option<MacAddr8>,
        nwk_addr_remote_dev: Option<u16>,
        num_assoc_dev: Option<u8>,
        start_index: Option<u8>,
        nwk_addr_assoc_dev_list: Box<[u16]>,
    }
    constructor {
        /// Creates a new Network Address Response.
        #[must_use]
        pub fn new(response: Result<NwkAddrRspResponse, Status>) -> Self {
            match response {
                Ok(NwkAddrRspResponse::Single {
                    ieee_addr_remote_dev,
                    nwk_addr_remote_dev,
                }) => Self {
                    status: Status::Success.into(),
                    ieee_addr_remote_dev: Some(ieee_addr_remote_dev),
                    nwk_addr_remote_dev: Some(nwk_addr_remote_dev),
                    num_assoc_dev: None,
                    start_index: None,
                    nwk_addr_assoc_dev_list: Box::default(),
                },
                Ok(NwkAddrRspResponse::Extended {
                    ieee_addr_remote_dev,
                    nwk_addr_remote_dev,
                    start_index,
                    nwk_addr_assoc_dev_list,
                }) => Self {
                    status: Status::Success.into(),
                    ieee_addr_remote_dev: Some(ieee_addr_remote_dev),
                    nwk_addr_remote_dev: Some(nwk_addr_remote_dev),
                    num_assoc_dev: Some(u8::try_from(nwk_addr_assoc_dev_list.len()).unwrap_or(u8::MAX)),
                    start_index: if nwk_addr_assoc_dev_list.is_empty() {
                        None
                    } else {
                        Some(start_index)
                    },
                    nwk_addr_assoc_dev_list: (*nwk_addr_assoc_dev_list)
                        .into_iter()
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                },
                Err(status) => Self {
                    status: status.into(),
                    ieee_addr_remote_dev: None,
                    nwk_addr_remote_dev: None,
                    num_assoc_dev: None,
                    start_index: None,
                    nwk_addr_assoc_dev_list: Box::default(),
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
    }
    le_stream {
        from {
            fn from_le_stream<T>(mut bytes: T) -> Option<Self>
            where
                T: Iterator<Item = u8>,
            {
                let status = <u8 as le_stream::FromLeStream>::from_le_stream(&mut bytes)?;
                let ieee_addr_remote_dev =
                    <Option<MacAddr8> as le_stream::FromLeStream>::from_le_stream(&mut bytes)?;
                let nwk_addr_remote_dev = if ieee_addr_remote_dev.is_some() {
                    Some(<u16 as le_stream::FromLeStream>::from_le_stream(&mut bytes)?)
                } else {
                    None
                };
                let num_assoc_dev =
                    <Option<u8> as le_stream::FromLeStream>::from_le_stream(&mut bytes)?;
                let start_index = if num_assoc_dev.is_some_and(|num_assoc_dev| num_assoc_dev > 0) {
                    Some(<u8 as le_stream::FromLeStream>::from_le_stream(&mut bytes)?)
                } else {
                    None
                };
                let mut nwk_addr_assoc_dev_list =
                    Vec::with_capacity(num_assoc_dev.map_or(0, usize::from));

                for _ in 0..num_assoc_dev.unwrap_or_default() {
                    nwk_addr_assoc_dev_list
                        .push(<u16 as le_stream::FromLeStream>::from_le_stream(&mut bytes)?);
                }

                Some(Self {
                    status,
                    ieee_addr_remote_dev,
                    nwk_addr_remote_dev,
                    num_assoc_dev,
                    start_index,
                    nwk_addr_assoc_dev_list: nwk_addr_assoc_dev_list.into_boxed_slice(),
                })
            }
        }
        to {
            type Iter = std::vec::IntoIter<u8>;

            fn to_le_stream(self) -> Self::Iter {
                let mut bytes = Vec::new();

                bytes.extend(<u8 as le_stream::ToLeStream>::to_le_stream(self.status));
                bytes.extend(<Option<MacAddr8> as le_stream::ToLeStream>::to_le_stream(
                    self.ieee_addr_remote_dev,
                ));
                bytes.extend(<Option<u16> as le_stream::ToLeStream>::to_le_stream(
                    self.nwk_addr_remote_dev,
                ));

                if let Some(num_assoc_dev) = self.num_assoc_dev {
                    bytes.extend(<u8 as le_stream::ToLeStream>::to_le_stream(num_assoc_dev));

                    if num_assoc_dev > 0 {
                        bytes.extend(<u8 as le_stream::ToLeStream>::to_le_stream(
                            self.start_index.unwrap_or_default(),
                        ));

                        for nwk_addr_assoc_dev in self.nwk_addr_assoc_dev_list {
                            bytes.extend(<u16 as le_stream::ToLeStream>::to_le_stream(
                                nwk_addr_assoc_dev,
                            ));
                        }
                    }
                }

                bytes.into_iter()
            }
        }
    }
}
