use apis_saltans_core::types::tlv::Tlv;

crate::zdp_command! {
    /// Security Start Key Negotiation Request.
    SecurityStartKeyNegotiationReq => Security_Start_Key_Negotiation_req;
    cluster_id: 0x0040;
    group: Security;
    fields {
        tlvs: Box<[Tlv]>,
    }
    getters {
    }
}
