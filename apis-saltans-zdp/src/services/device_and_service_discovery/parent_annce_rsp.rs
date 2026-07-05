use apis_saltans_core::ByteSizedVec;
use macaddr::MacAddr8;

use crate::Status;

crate::zdp_command! {
    /// Parent Announce Response.
    ParentAnnceRsp => Parent_annce_rsp;
    cluster_id: 0x801f;
    group: DeviceAndServiceDiscovery;
    fields {
        status: u8,
        child_info: ByteSizedVec<MacAddr8>,
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
