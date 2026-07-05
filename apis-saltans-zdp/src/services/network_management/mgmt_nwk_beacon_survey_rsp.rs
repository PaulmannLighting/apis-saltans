use apis_saltans_core::types::tlv::Tlv;

use crate::Status;

crate::zdp_command! {
    /// Management Network Beacon Survey Response.
    MgmtNwkBeaconSurveyRsp => Mgmt_NWK_Beacon_Survey_rsp;
    cluster_id: 0x803c;
    group: NetworkManagement;
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
