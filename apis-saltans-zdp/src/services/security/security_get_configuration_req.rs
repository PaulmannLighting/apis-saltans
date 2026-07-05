use crate::ByteSizedVec;

crate::zdp_command! {
    /// Security Get Configuration Request.
    SecurityGetConfigurationReq => Security_Get_Configuration_req;
    cluster_id: 0x0044;
    group: Security;
    fields {
        tlv_ids: ByteSizedVec<u8>,
    }
    getters {
    }
}
