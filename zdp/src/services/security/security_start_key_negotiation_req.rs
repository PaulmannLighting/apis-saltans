use crate::SecurityStartKeyNegotiationRsp;
use crate::storage::TlvSlice;

crate::zdp_command! {
    /// Security Start Key Negotiation Request.
    SecurityStartKeyNegotiationReq => Security_Start_Key_Negotiation_req;
    cluster_id: 0x0040;
    group: Security;
    response: SecurityStartKeyNegotiationRsp;
    fields {
        tlvs: TlvSlice,
    }
    getters {
    }
}
