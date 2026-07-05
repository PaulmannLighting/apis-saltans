crate::zdp_command! {
    /// Security Decommission Response.
    derive { Copy }
    SecurityDecommissionRsp => Security_Decommission_rsp;
    cluster_id: 0x8046;
    group: Security;
    fields {
        status: u8,
    }
    getters {
    }
}
