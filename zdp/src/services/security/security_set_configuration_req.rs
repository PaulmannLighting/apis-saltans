use crate::SecuritySetConfigurationRsp;
use crate::storage::TlvSlice;

crate::zdp_command! {
    /// Security Set Configuration Request.
    SecuritySetConfigurationReq => Security_Set_Configuration_req;
    cluster_id: 0x0043;
    group: Security;
    response: SecuritySetConfigurationRsp;
    fields {
        tlvs: TlvSlice,
    }
    getters {
    }
}
