crate::zdp_command! {
    /// Management Leave Response.
    derive { Copy }
    MgmtLeaveRsp => Mgmt_Leave_rsp;
    cluster_id: 0x8034;
    group: NetworkManagement;
    fields {
        status: u8,
    }
    getters {
    }
}
