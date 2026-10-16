use crate::SecurityDecommissionRsp;
use crate::storage::TlvSlice;

crate::zdp_command! {
    /// Security Decommission Request.
    SecurityDecommissionReq => Security_Decommission_req;
    cluster_id: 0x0046;
    group: Security;
    response: SecurityDecommissionRsp;
    fields {
        tlvs: TlvSlice,
    }
    getters {
    }
}
