use apis_saltans_core::types::tlv::Tlv;

crate::zdp_command! {
    /// Security Set Configuration Response.
    SecuritySetConfigurationRsp => Security_Set_Configuration_rsp;
    cluster_id: 0x8043;
    group: Security;
    fields {
        overall_status: u8,
        tlvs: Box<[Tlv]>,
    }
    getters {
    }
}
