use crate::{ByteSizedVec, Status};

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
