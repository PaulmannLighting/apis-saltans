use apis_saltans_core::types::tlv::Tlv;

use crate::Status;

crate::zdp_command! {
    /// Security Set Configuration Response.
    SecuritySetConfigurationRsp => Security_Set_Configuration_rsp;
    cluster_id: 0x8043;
    group: Security;
    fields {
        overall_status: u8,
        tlvs: Box<[Tlv]>,
    }
    getters {
        /// Return the overall status of the response.
        ///
        /// # Errors
        ///
        /// Returns the raw status code if the conversion to a [`Status`] fails.
        pub fn overall_status(&self) -> Result<Status, u8> {
            self.overall_status.try_into()
        }
    }
}
