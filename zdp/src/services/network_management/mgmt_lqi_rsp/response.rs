use zb_core::ByteSizedVec;

/// Successful Management LQI Response payload.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct MgmtLqiRspPayload {
    /// Total neighbor table entries on the remote device.
    pub neighbor_table_entries: u8,
    /// Starting index for the returned list.
    pub start_index: u8,
    /// Neighbor table list bytes.
    pub neighbor_table_list: ByteSizedVec<u8>,
}
