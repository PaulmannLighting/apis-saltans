crate::zdp_command! {
    /// Unbind Response.
    derive { Copy }
    UnbindRsp => Unbind_rsp;
    cluster_id: 0x8022;
    group: BindManagement;
    fields {
        status: u8,
    }
    getters {
    }
}
