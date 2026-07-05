crate::zdp_command! {
    /// Power Descriptor Response.
    derive { Copy }
    PowerDescRsp => Power_Desc_rsp;
    cluster_id: 0x8003;
    group: DeviceAndServiceDiscovery;
    fields {
        status: u8,
        nwk_addr_of_interest: u16,
        power_descriptor: Option<u16>,
    }
    getters {
    }
}
