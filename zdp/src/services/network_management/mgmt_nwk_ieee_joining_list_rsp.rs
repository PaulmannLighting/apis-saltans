use zb_core::{ByteSizedVec, IeeeAddress};

pub use self::response::{
    JoiningPolicy, MgmtNwkIeeeJoiningListRspEntries, MgmtNwkIeeeJoiningListRspPayload,
};
use crate::Status;

mod response;

crate::zdp_command! {
    /// Management Network IEEE Joining List Response.
    MgmtNwkIeeeJoiningListRsp => Mgmt_NWK_IEEE_Joining_List_rsp;
    cluster_id: 0x803a;
    group: NetworkManagement;
    fields {
        status: u8,
        ieee_joining_list_update_id: Option<u8>,
        joining_policy: u8,
        ieee_joining_list_total: u8,
        start_index: u8,
        ieee_joining_list: ByteSizedVec<IeeeAddress>,
    }
    constructor {
        /// Creates a new `MgmtNwkIeeeJoiningListRsp`.
        #[must_use]
        pub fn new(response: Result<MgmtNwkIeeeJoiningListRspPayload, Status>) -> Self {
            match response {
                Ok(MgmtNwkIeeeJoiningListRspPayload {
                    ieee_joining_list_update_id,
                    joining_policy,
                    entries,
                }) => {
                    let (
                        ieee_joining_list_total,
                        start_index,
                        ieee_joining_list,
                    ) = entries.map_or_else(
                        || (0, 0, ByteSizedVec::new()),
                        |entries| {
                            (
                                entries.ieee_joining_list_total.into(),
                                entries.start_index,
                                *entries.ieee_joining_list,
                            )
                        },
                    );

                    Self {
                        status: Status::Success.into(),
                        ieee_joining_list_update_id: Some(ieee_joining_list_update_id),
                        joining_policy: joining_policy.into(),
                        ieee_joining_list_total,
                        start_index,
                        ieee_joining_list,
                    }
                }
                Err(status) => Self {
                    status: status.into(),
                    ieee_joining_list_update_id: None,
                    joining_policy: 0,
                    ieee_joining_list_total: 0,
                    start_index: 0,
                    ieee_joining_list: ByteSizedVec::new(),
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
                    let ieee_joining_list_update_id =
                        <u8 as le_stream::FromLeStream>::from_le_stream(&mut bytes)?;
                    let joining_policy =
                        <u8 as le_stream::FromLeStream>::from_le_stream(&mut bytes)?;
                    let ieee_joining_list_total =
                        <u8 as le_stream::FromLeStream>::from_le_stream(&mut bytes)?;

                    if ieee_joining_list_total == 0 {
                        Some(Self {
                            status,
                            ieee_joining_list_update_id: Some(ieee_joining_list_update_id),
                            joining_policy,
                            ieee_joining_list_total,
                            start_index: 0,
                            ieee_joining_list: ByteSizedVec::new(),
                        })
                    } else {
                        Some(Self {
                            status,
                            ieee_joining_list_update_id: Some(ieee_joining_list_update_id),
                            joining_policy,
                            ieee_joining_list_total,
                            start_index: <u8 as le_stream::FromLeStream>::from_le_stream(&mut bytes)?,
                            ieee_joining_list: <ByteSizedVec<IeeeAddress> as le_stream::FromLeStream>::from_le_stream(&mut bytes)?,
                        })
                    }
                } else {
                    Some(Self {
                        status,
                        ieee_joining_list_update_id: None,
                        joining_policy: 0,
                        ieee_joining_list_total: 0,
                        start_index: 0,
                        ieee_joining_list: ByteSizedVec::new(),
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
                    bytes.extend(<Option<u8> as le_stream::ToLeStream>::to_le_stream(
                        self.ieee_joining_list_update_id,
                    ));
                    bytes.extend(<u8 as le_stream::ToLeStream>::to_le_stream(self.joining_policy));
                    bytes.extend(<u8 as le_stream::ToLeStream>::to_le_stream(
                        self.ieee_joining_list_total,
                    ));

                    if self.ieee_joining_list_total > 0 {
                        bytes.extend(<u8 as le_stream::ToLeStream>::to_le_stream(self.start_index));
                        bytes.extend(
                            <ByteSizedVec<IeeeAddress> as le_stream::ToLeStream>::to_le_stream(
                                self.ieee_joining_list,
                            ),
                        );
                    }
                }

                bytes.into_iter()
            }
        }
    }
}
