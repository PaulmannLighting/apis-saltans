use zb_core::types::tlv::Tlv;

crate::zdp_command! {
    /// Security Set Configuration Request.
    SecuritySetConfigurationReq => Security_Set_Configuration_req;
    cluster_id: 0x0043;
    group: Security;
    response: crate::SecuritySetConfigurationRsp;
    fields {
        tlvs: Box<[Tlv]>,
    }
    getters {
    }
}
