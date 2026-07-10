use zb_core::ByteSizedVec;

/// Successful Management Binding Table Response payload.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct MgmtBindRspPayload {
    /// Total binding table entries on the remote device.
    pub binding_table_entries: u8,
    /// Starting index for the returned list.
    pub start_index: u8,
    /// Binding table list bytes.
    pub binding_table_list: ByteSizedVec<u8>,
}
