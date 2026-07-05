use apis_saltans_core::types::tlv::Tlv;

crate::zdp_command! {
    /// Security Start Key Negotiation Response.
    SecurityStartKeyNegotiationRsp => Security_Start_Key_Negotiation_rsp;
    cluster_id: 0x8040;
    group: Security;
    fields {
        status: u8,
        tlvs: Box<[Tlv]>,
    }
    getters {
    }
}
