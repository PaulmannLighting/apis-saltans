use zb_core::types::tlv::Tlv;

use crate::SecuritySetConfigurationRsp;

crate::zdp_command! {
    /// Security Set Configuration Request.
    SecuritySetConfigurationReq => Security_Set_Configuration_req;
    cluster_id: 0x0043;
    group: Security;
    response: SecuritySetConfigurationRsp;
    fields {
        tlvs: Box<[Tlv]>,
    }
    getters {
    }
}
