use apis_saltans_core::types::tlv::Tlv;

crate::zdp_command! {
    /// Security Get Authentication Level Response.
    SecurityGetAuthenticationLevelRsp => Security_Get_Authentication_Level_rsp;
    cluster_id: 0x8042;
    group: Security;
    fields {
        status: u8,
        tlvs: Box<[Tlv]>,
    }
    getters {
    }
}
