use std::num::NonZeroU8;

use apis_saltans_core::{ByteSizedVec, IeeeAddress};

/// ZDO joining policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum JoiningPolicy {
    /// Any device is allowed to join.
    AllJoin = 0x00,
    /// Only devices on the IEEE joining list are allowed to join.
    IeeeListJoin = 0x01,
    /// No device is allowed to join.
    NoJoin = 0x02,
}

impl From<JoiningPolicy> for u8 {
    fn from(value: JoiningPolicy) -> Self {
        value as Self
    }
}

/// Successful Management Network IEEE Joining List Response payload.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct MgmtNwkIeeeJoiningListRspPayload {
    /// Joining list update ID.
    pub ieee_joining_list_update_id: u8,
    /// Current joining policy.
    pub joining_policy: JoiningPolicy,
    /// Present only when the IEEE joining list total is non-zero.
    pub entries: Option<MgmtNwkIeeeJoiningListRspEntries>,
}

/// IEEE joining list entries in a successful response.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct MgmtNwkIeeeJoiningListRspEntries {
    /// Total number of IEEE joining addresses.
    pub ieee_joining_list_total: NonZeroU8,
    /// Starting index of this response segment.
    pub start_index: u8,
    /// IEEE joining addresses in this response segment.
    pub ieee_joining_list: Box<ByteSizedVec<IeeeAddress>>,
}
