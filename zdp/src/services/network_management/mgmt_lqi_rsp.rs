use apis_saltans_core::ByteSizedVec;

pub use self::response::MgmtLqiRspPayload;
use crate::Status;

mod response;

crate::zdp_command! {
    /// Management LQI Response.
    MgmtLqiRsp => Mgmt_Lqi_rsp;
    cluster_id: 0x8031;
    group: NetworkManagement;
    fields {
        status: u8,
        neighbor_table_entries: u8,
        start_index: u8,
        neighbor_table_list: ByteSizedVec<u8>,
    }
    constructor {
        /// Creates a new `MgmtLqiRsp`.
        #[must_use]
        pub fn new(response: Result<MgmtLqiRspPayload, Status>) -> Self {
            match response {
                Ok(response) => Self {
                    status: Status::Success.into(),
                    neighbor_table_entries: response.neighbor_table_entries,
                    start_index: response.start_index,
                    neighbor_table_list: response.neighbor_table_list,
                },
                Err(status) => Self {
                    status: status.into(),
                    neighbor_table_entries: 0,
                    start_index: 0,
                    neighbor_table_list: ByteSizedVec::new(),
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

                if status == Status::Success as u8 {
                    Some(Self {
                        status,
                        neighbor_table_entries: <u8 as le_stream::FromLeStream>::from_le_stream(&mut bytes)?,
                        start_index: <u8 as le_stream::FromLeStream>::from_le_stream(&mut bytes)?,
                        neighbor_table_list: <ByteSizedVec<u8> as le_stream::FromLeStream>::from_le_stream(&mut bytes)?,
                    })
                } else {
                    Some(Self {
                        status,
                        neighbor_table_entries: 0,
                        start_index: 0,
                        neighbor_table_list: ByteSizedVec::new(),
                    })
                }
            }
        }
        to {
            type Iter = std::vec::IntoIter<u8>;

            fn to_le_stream(self) -> Self::Iter {
                let mut bytes = Vec::new();

                bytes.extend(<u8 as le_stream::ToLeStream>::to_le_stream(self.status));

                if self.status == Status::Success as u8 {
                    bytes.extend(<u8 as le_stream::ToLeStream>::to_le_stream(self.neighbor_table_entries));
                    bytes.extend(<u8 as le_stream::ToLeStream>::to_le_stream(self.start_index));
                    bytes.extend(<ByteSizedVec<u8> as le_stream::ToLeStream>::to_le_stream(self.neighbor_table_list));
                }

                bytes.into_iter()
            }
        }
    }
}
