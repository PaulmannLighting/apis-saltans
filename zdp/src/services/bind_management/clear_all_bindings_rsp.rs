use crate::Status;

crate::zdp_command! {
    /// Clear All Bindings Response.
    derive { Copy }
    ClearAllBindingsRsp => Clear_All_Bindings_rsp;
    cluster_id: 0x802b;
    group: BindManagement;
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
