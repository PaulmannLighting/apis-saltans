use macaddr::MacAddr8;

use crate::ByteSizedVec;

crate::zdp_command! {
    /// Management Network IEEE Joining List Response.
    MgmtNwkIeeeJoiningListRsp => Mgmt_NWK_IEEE_Joining_List_rsp;
    cluster_id: 0x803a;
    group: NetworkManagement;
    fields {
        status: u8,
        ieee_joining_list_update_id: Option<u8>,
        joining_policy: Option<u8>,
        ieee_joining_list_total: Option<u8>,
        start_index: Option<u8>,
        ieee_joining_list: Option<ByteSizedVec<MacAddr8>>,
    }
    getters {
    }
}
