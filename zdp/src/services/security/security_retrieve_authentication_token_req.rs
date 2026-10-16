use crate::SecurityRetrieveAuthenticationTokenRsp;
use crate::storage::TlvSlice;

crate::zdp_command! {
    /// Security Retrieve Authentication Token Request.
    SecurityRetrieveAuthenticationTokenReq => Security_Retrieve_Authentication_Token_req;
    cluster_id: 0x0041;
    group: Security;
    response: SecurityRetrieveAuthenticationTokenRsp;
    fields {
        tlvs: TlvSlice,
    }
    getters {
    }
}
