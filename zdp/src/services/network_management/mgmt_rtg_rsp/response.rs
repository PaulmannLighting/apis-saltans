use zb_core::ByteSizedVec;

/// Successful Management Routing Table Response payload.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct MgmtRtgRspPayload {
    /// Total routing table entries on the remote device.
    pub routing_table_entries: u8,
    /// Starting index for the returned list.
    pub start_index: u8,
    /// Routing table list bytes.
    pub routing_table_list: ByteSizedVec<u8>,
}
