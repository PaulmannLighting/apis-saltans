use crate::Status;

crate::zdp_command! {
    /// Management Routing Table Response.
    MgmtRtgRsp => Mgmt_Rtg_rsp;
    cluster_id: 0x8032;
    group: NetworkManagement;
    fields {
        status: u8,
        routing_table_entries: u8,
        start_index: u8,
        routing_table_list_count: u8,
        routing_table_list: Box<[u8]>,
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
}
