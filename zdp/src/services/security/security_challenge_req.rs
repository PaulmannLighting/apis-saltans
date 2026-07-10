use zb_core::types::tlv::Tlv;

crate::zdp_command! {
    /// Security Challenge Request.
    SecurityChallengeReq => Security_Challenge_req;
    cluster_id: 0x0047;
    group: Security;
    response: crate::SecurityChallengeRsp;
    fields {
        tlvs: Box<[Tlv]>,
    }
    getters {
    }
}
