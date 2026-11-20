pub use self::response::MgmtBindRspPayload;
use super::table::management_table_response;

mod response;

management_table_response! {
    /// Management Binding Table response.
    MgmtBindRsp => Mgmt_Bind_rsp;
    cluster_id: 0x8033;
    layout: Binding;
    payload: MgmtBindRspPayload;
    fields: binding_table_entries, binding_table_list;
}
