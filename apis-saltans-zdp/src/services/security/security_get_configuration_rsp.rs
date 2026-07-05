use apis_saltans_core::types::tlv::Tlv;

crate::zdp_command! {
    /// Security Get Configuration Response.
    SecurityGetConfigurationRsp => Security_Get_Configuration_rsp;
    cluster_id: 0x8044;
    group: Security;
    fields {
        overall_status: u8,
        tlvs: Box<[Tlv]>,
    }
    getters {
    }
}
