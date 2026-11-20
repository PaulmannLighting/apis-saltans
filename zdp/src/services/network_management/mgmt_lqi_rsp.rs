pub use self::response::MgmtLqiRspPayload;
use super::table::management_table_response;

mod response;

management_table_response! {
    /// Management Link Quality Indicator response.
    MgmtLqiRsp => Mgmt_Lqi_rsp;
    cluster_id: 0x8031;
    layout: Neighbor;
    payload: MgmtLqiRspPayload;
    fields: neighbor_table_entries, neighbor_table_list;
}
