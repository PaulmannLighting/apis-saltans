use apis_saltans_core::types::tlv::Tlv;

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
    }
}
