use crate::SecurityGetAuthenticationLevelRsp;
use crate::storage::TlvSlice;

crate::zdp_command! {
    /// Security Get Authentication Level Request.
    SecurityGetAuthenticationLevelReq => Security_Get_Authentication_Level_req;
    cluster_id: 0x0042;
    group: Security;
    response: SecurityGetAuthenticationLevelRsp;
    fields {
        tlvs: TlvSlice,
    }
    getters {
    }
}
