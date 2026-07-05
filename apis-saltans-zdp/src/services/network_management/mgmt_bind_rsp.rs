crate::zdp_command! {
    /// Management Binding Table Response.
    MgmtBindRsp => Mgmt_Bind_rsp;
    cluster_id: 0x8033;
    group: NetworkManagement;
    fields {
        status: u8,
        binding_table_entries: u8,
        start_index: u8,
        binding_table_list_count: u8,
        binding_table_list: Box<[u8]>,
    }
    getters {
    }
}
