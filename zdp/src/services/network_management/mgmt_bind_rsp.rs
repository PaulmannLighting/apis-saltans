#[cfg(feature = "alloc")]
use alloc::{vec::IntoIter, vec::Vec};

use le_stream::FromLeStream;
#[cfg(feature = "alloc")]
use le_stream::ToLeStream;
use zb_core::ByteSizedVec;

pub use self::response::MgmtBindRspPayload;
use crate::Status;

mod response;

crate::zdp_command! {
    /// Management Binding Table Response.
    MgmtBindRsp => Mgmt_Bind_rsp;
    cluster_id: 0x8033;
    group: NetworkManagement;
    fields {
        status: u8,
        binding_table_entries: u8,
        start_index: u8,
        binding_table_list: ByteSizedVec<u8>,
    }
    constructor {
        /// Creates a new `MgmtBindRsp`.
        #[must_use]
        pub fn new(response: Result<MgmtBindRspPayload, Status>) -> Self {
            match response {
                Ok(response) => Self {
                    status: Status::Success.into(),
                    binding_table_entries: response.binding_table_entries,
                    start_index: response.start_index,
                    binding_table_list: response.binding_table_list,
                },
                Err(status) => Self {
                    status: status.into(),
                    binding_table_entries: 0,
                    start_index: 0,
                    binding_table_list: ByteSizedVec::new(),
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
                let status = <u8 as FromLeStream>::from_le_stream(&mut bytes)?;

                if status == Status::Success as u8 {
                    Some(Self {
                        status,
                        binding_table_entries: <u8 as FromLeStream>::from_le_stream(&mut bytes)?,
                        start_index: <u8 as FromLeStream>::from_le_stream(&mut bytes)?,
                        binding_table_list: <ByteSizedVec<u8> as FromLeStream>::from_le_stream(&mut bytes)?,
                    })
                } else {
                    Some(Self {
                        status,
                        binding_table_entries: 0,
                        start_index: 0,
                        binding_table_list: ByteSizedVec::new(),
                    })
                }
            }
        }
        to {
            #[cfg(feature = "alloc")]
            type Iter = IntoIter<u8>;
            #[cfg(not(feature = "alloc"))]
            type Iter = crate::storage::Stream<<ByteSizedVec<u8> as IntoIterator>::IntoIter>;

            #[cfg(feature = "alloc")]
            fn to_le_stream(self) -> Self::Iter {
                let mut bytes = Vec::new();

                bytes.extend(<u8 as ToLeStream>::to_le_stream(self.status));

                if self.status == Status::Success as u8 {
                    bytes.extend(<u8 as ToLeStream>::to_le_stream(self.binding_table_entries));
                    bytes.extend(<u8 as ToLeStream>::to_le_stream(self.start_index));
                    bytes.extend(<ByteSizedVec<u8> as ToLeStream>::to_le_stream(self.binding_table_list));
                }

                bytes.into_iter()
            }

            #[cfg(not(feature = "alloc"))]
            fn to_le_stream(self) -> Self::Iter {
                let mut prefix = crate::storage::Prefix::new();
                prefix.extend(self.status.to_le_stream());
                let mut items = self.binding_table_list;
                if self.status == Status::Success as u8 {
                    prefix.extend(self.binding_table_entries.to_le_stream());
                    prefix.extend(self.start_index.to_le_stream());
                    prefix.extend(u8::try_from(items.len()).expect("byte-sized list length fits u8").to_le_stream());
                } else { items.clear(); }
                crate::storage::Stream::new(&prefix, items.into_iter())
            }
        }
    }
}
