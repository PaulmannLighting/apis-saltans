use apis_saltans_core::types::tlv::Tlv;

crate::zdp_command! {
    /// Security Start Key Update Request.
    SecurityStartKeyUpdateReq => Security_Start_Key_Update_req;
    cluster_id: 0x0045;
    group: Security;
    fields {
        tlvs: Box<[Tlv]>,
    }
    getters {
    }
}
