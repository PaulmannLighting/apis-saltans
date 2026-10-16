use crate::SecurityChallengeRsp;
use crate::storage::TlvSlice;

crate::zdp_command! {
    /// Security Challenge Request.
    SecurityChallengeReq => Security_Challenge_req;
    cluster_id: 0x0047;
    group: Security;
    response: SecurityChallengeRsp;
    fields {
        tlvs: TlvSlice,
    }
    getters {
    }
}
