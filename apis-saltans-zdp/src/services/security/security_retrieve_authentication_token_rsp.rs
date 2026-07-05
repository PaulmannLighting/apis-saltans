use apis_saltans_core::types::tlv::Tlv;

crate::zdp_command! {
    /// Security Retrieve Authentication Token Response.
    SecurityRetrieveAuthenticationTokenRsp => Security_Retrieve_Authentication_Token_rsp;
    cluster_id: 0x8041;
    group: Security;
    fields {
        status: u8,
        tlvs: Box<[Tlv]>,
    }
    getters {
    }
}
