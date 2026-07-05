crate::zdp_command! {
    /// System Server Discovery Response.
    derive { Copy }
    SystemServerDiscoveryRsp => System_Server_Discovery_rsp;
    cluster_id: 0x8015;
    group: DeviceAndServiceDiscovery;
    fields {
        status: u8,
        server_mask: u16,
    }
    getters {
    }
}
