use zb_core::ByteSizedVec;

use crate::Status;

crate::zdp_command! {
    /// Management Network Update Notify.
    MgmtNwkUpdateNotify => Mgmt_NWK_Update_notify;
    cluster_id: 0x8038;
    group: NetworkManagement;
    fields {
        status: u8,
        scanned_channels: u32,
        total_transmissions: u16,
        transmission_failures: u16,
        energy_values: ByteSizedVec<u8>,
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
