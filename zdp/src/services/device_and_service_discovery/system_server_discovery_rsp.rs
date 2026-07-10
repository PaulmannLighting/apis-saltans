use crate::Status;

crate::zdp_command! {
    /// System Server Discovery Response.
    derive { Copy }
    SystemServerDiscoveryRsp => System_Server_Discovery_rsp;
    cluster_id: 0x8015;
    group: DeviceAndServiceDiscovery;
    fields {
        status: u8,
        server_mask: u16,
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
