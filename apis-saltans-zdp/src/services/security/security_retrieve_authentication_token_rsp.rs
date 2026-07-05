use apis_saltans_core::types::tlv::Tlv;

use crate::Status;

crate::zdp_command! {
    /// Security Retrieve Authentication Token Response.
    SecurityRetrieveAuthenticationTokenRsp => Security_Retrieve_Authentication_Token_rsp;
    cluster_id: 0x8041;
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
