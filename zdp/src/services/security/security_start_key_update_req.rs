use crate::SecurityStartKeyUpdateRsp;
use crate::storage::TlvSlice;

crate::zdp_command! {
    /// Security Start Key Update Request.
    SecurityStartKeyUpdateReq => Security_Start_Key_Update_req;
    cluster_id: 0x0045;
    group: Security;
    response: SecurityStartKeyUpdateRsp;
    fields {
        tlvs: TlvSlice,
    }
    getters {
    }
}
