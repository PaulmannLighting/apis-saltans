use crate::storage::TlvSlice;

crate::zdp_command! {
    /// Security Challenge Response.
    SecurityChallengeRsp => Security_Challenge_rsp;
    cluster_id: 0x8047;
    group: Security;
    fields {
        tlvs: TlvSlice,
    }
    getters {
    }
}
