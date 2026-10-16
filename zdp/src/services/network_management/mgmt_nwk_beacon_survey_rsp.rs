use crate::Status;
use crate::storage::TlvSlice;

crate::zdp_command! {
    /// Management Network Beacon Survey Response.
    MgmtNwkBeaconSurveyRsp => Mgmt_NWK_Beacon_Survey_rsp;
    cluster_id: 0x803c;
    group: NetworkManagement;
    fields {
        status: u8,
        tlvs: TlvSlice,
    }
    constructor {
        /// Creates a new `MgmtNwkBeaconSurveyRsp`.
        #[must_use]
        pub fn new(response: Result<TlvSlice, Status>) -> Self {
            match response {
                Ok(tlvs) => Self {
                    status: Status::Success.into(),
                    tlvs,
                },
                Err(status) => Self {
                    status: status.into(),
                    tlvs: TlvSlice::default(),
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
