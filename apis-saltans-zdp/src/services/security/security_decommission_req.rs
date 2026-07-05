use apis_saltans_core::types::tlv::Tlv;

crate::zdp_command! {
    /// Security Decommission Request.
    SecurityDecommissionReq => Security_Decommission_req;
    cluster_id: 0x0046;
    group: Security;
    fields {
        tlvs: Box<[Tlv]>,
    }
    getters {
    }
}
