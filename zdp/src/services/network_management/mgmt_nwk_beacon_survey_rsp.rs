use zb_core::types::tlv::Tlv;

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
    constructor {
        /// Creates a new `MgmtNwkBeaconSurveyRsp`.
        #[must_use]
        pub fn new(response: Result<Box<[Tlv]>, Status>) -> Self {
            match response {
                Ok(tlvs) => Self {
                    status: Status::Success.into(),
                    tlvs,
                },
                Err(status) => Self {
                    status: status.into(),
                    tlvs: Box::default(),
                },
            }
        }
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
