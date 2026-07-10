use apis_saltans_core::types::tlv::Tlv;

crate::zdp_command! {
    /// Security Challenge Response.
    SecurityChallengeRsp => Security_Challenge_rsp;
    cluster_id: 0x8047;
    group: Security;
    fields {
        tlvs: Box<[Tlv]>,
    }
    getters {
    }
}
