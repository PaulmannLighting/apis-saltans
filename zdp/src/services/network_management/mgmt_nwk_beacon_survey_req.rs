use le_stream::{FromLeStream, ToLeStream};
use zigbee::Cluster;
use zigbee::types::tlv::Tlv;

use crate::Service;

/// Management Network Beacon Survey Request.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct MgmtNwkBeaconSurveyReq {
    tlvs: Vec<Tlv>,
}

impl Cluster for MgmtNwkBeaconSurveyReq {
    const ID: u16 = 0x003c;
}

impl Service for MgmtNwkBeaconSurveyReq {
    const NAME: &'static str = "Mgmt_NWK_Beacon_Survey_req";
}
