use zb_core::types::tlv::Tlv;

use crate::SecurityStartKeyUpdateRsp;

crate::zdp_command! {
    /// Security Start Key Update Request.
    SecurityStartKeyUpdateReq => Security_Start_Key_Update_req;
    cluster_id: 0x0045;
    group: Security;
    response: SecurityStartKeyUpdateRsp;
    fields {
        tlvs: Box<[Tlv]>,
    }
    getters {
    }
}
