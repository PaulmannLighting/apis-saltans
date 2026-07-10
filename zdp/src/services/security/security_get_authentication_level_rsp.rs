use zb_core::types::tlv::Tlv;

use crate::Status;

crate::zdp_command! {
    /// Security Get Authentication Level Response.
    SecurityGetAuthenticationLevelRsp => Security_Get_Authentication_Level_rsp;
    cluster_id: 0x8042;
    group: Security;
    fields {
        status: u8,
        tlvs: Box<[Tlv]>,
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
