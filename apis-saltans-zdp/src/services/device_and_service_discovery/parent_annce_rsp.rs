use macaddr::MacAddr8;

use crate::ByteSizedVec;

crate::zdp_command! {
    /// Parent Announce Response.
    ParentAnnceRsp => Parent_annce_rsp;
    cluster_id: 0x801f;
    group: DeviceAndServiceDiscovery;
    fields {
        status: u8,
        child_info: ByteSizedVec<MacAddr8>,
    }
    getters {
    }
}
