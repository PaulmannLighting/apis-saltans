use zb_core::types::tlv::Tlv;

crate::zdp_command! {
    /// Security Decommission Request.
    SecurityDecommissionReq => Security_Decommission_req;
    cluster_id: 0x0046;
    group: Security;
    response: crate::SecurityDecommissionRsp;
    fields {
        tlvs: Box<[Tlv]>,
    }
    getters {
    }
}
