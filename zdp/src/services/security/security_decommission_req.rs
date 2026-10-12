use zb_core::types::tlv::Tlv;

use crate::SecurityDecommissionRsp;

crate::zdp_command! {
    /// Security Decommission Request.
    SecurityDecommissionReq => Security_Decommission_req;
    cluster_id: 0x0046;
    group: Security;
    response: SecurityDecommissionRsp;
    fields {
        tlvs: Box<[Tlv]>,
    }
    getters {
    }
}
