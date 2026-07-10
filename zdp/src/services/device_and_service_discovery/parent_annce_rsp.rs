use zb_core::{ByteSizedVec, IeeeAddress};

use crate::Status;

crate::zdp_command! {
    /// Parent Announce Response.
    ParentAnnceRsp => Parent_annce_rsp;
    cluster_id: 0x801f;
    group: DeviceAndServiceDiscovery;
    fields {
        status: u8,
        child_info: ByteSizedVec<IeeeAddress>,
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
