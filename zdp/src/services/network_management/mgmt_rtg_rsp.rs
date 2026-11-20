pub use self::response::MgmtRtgRspPayload;
use super::table::management_table_response;

mod response;

management_table_response! {
    /// Management Routing Table response.
    MgmtRtgRsp => Mgmt_Rtg_rsp;
    cluster_id: 0x8032;
    layout: Routing;
    payload: MgmtRtgRspPayload;
    fields: routing_table_entries, routing_table_list;
}
