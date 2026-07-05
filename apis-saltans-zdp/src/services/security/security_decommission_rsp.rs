use crate::Status;

crate::zdp_command! {
    /// Security Decommission Response.
    derive { Copy }
    SecurityDecommissionRsp => Security_Decommission_rsp;
    cluster_id: 0x8046;
    group: Security;
    fields {
        status: u8,
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
