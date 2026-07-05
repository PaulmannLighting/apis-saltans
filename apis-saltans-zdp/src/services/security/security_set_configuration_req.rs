use apis_saltans_core::types::tlv::Tlv;

crate::zdp_command! {
    /// Security Set Configuration Request.
    SecuritySetConfigurationReq => Security_Set_Configuration_req;
    cluster_id: 0x0043;
    group: Security;
    fields {
        tlvs: Box<[Tlv]>,
    }
    getters {
    }
}
