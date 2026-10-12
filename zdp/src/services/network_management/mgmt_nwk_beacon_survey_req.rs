use std::fmt::Formatter;

use zb_core::types::tlv::Tlv;

use crate::MgmtNwkBeaconSurveyRsp;

crate::zdp_command! {
    /// Management Network Beacon Survey Request.
    MgmtNwkBeaconSurveyReq => Mgmt_NWK_Beacon_Survey_req;
    cluster_id: 0x003c;
    group: NetworkManagement;
    response: MgmtNwkBeaconSurveyRsp;
    fields {
        tlvs: Box<[Tlv]>,
    }
    getters {
    }
    display {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {{ tlvs: [", Self::NAME)?;

            let mut tlvs = self.tlvs.iter();
            if let Some(tlv) = tlvs.next() {
                write!(f, "{tlv:?}")?;

                for tlv in tlvs {
                    write!(f, ", {tlv:?}")?;
                }
            }

            write!(f, "] }}")
        }
    }
}
