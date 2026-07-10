use crate::Status;

crate::zdp_command! {
    /// Security Start Key Update Response.
    derive { Copy }
    SecurityStartKeyUpdateRsp => Security_Start_Key_Update_rsp;
    cluster_id: 0x8045;
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
