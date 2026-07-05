crate::zdp_command! {
    /// Clear All Bindings Response.
    derive { Copy }
    ClearAllBindingsRsp => Clear_All_Bindings_rsp;
    cluster_id: 0x802b;
    group: BindManagement;
    fields {
        status: u8,
    }
    getters {
    }
}
