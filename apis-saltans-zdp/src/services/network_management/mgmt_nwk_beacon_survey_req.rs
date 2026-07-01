use std::fmt::Display;

use apis_saltans_core::Cluster;
use apis_saltans_core::types::tlv::Tlv;
use le_stream::{FromLeStream, ToLeStream};

use crate::Service;

/// Management Network Beacon Survey Request.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct MgmtNwkBeaconSurveyReq {
    tlvs: Box<[Tlv]>,
}

impl Cluster for MgmtNwkBeaconSurveyReq {
    const ID: u16 = 0x003c;
}

impl Service for MgmtNwkBeaconSurveyReq {
    const NAME: &'static str = "Mgmt_NWK_Beacon_Survey_req";
}

impl Display for MgmtNwkBeaconSurveyReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
