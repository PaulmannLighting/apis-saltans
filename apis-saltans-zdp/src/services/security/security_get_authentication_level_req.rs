use apis_saltans_core::types::tlv::Tlv;

crate::zdp_command! {
    /// Security Get Authentication Level Request.
    SecurityGetAuthenticationLevelReq => Security_Get_Authentication_Level_req;
    cluster_id: 0x0042;
    group: Security;
    fields {
        tlvs: Box<[Tlv]>,
    }
    getters {
    }
}
