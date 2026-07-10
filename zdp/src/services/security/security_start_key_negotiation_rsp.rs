use zb_core::types::tlv::Tlv;

use crate::Status;

crate::zdp_command! {
    /// Security Start Key Negotiation Response.
    SecurityStartKeyNegotiationRsp => Security_Start_Key_Negotiation_rsp;
    cluster_id: 0x8040;
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
