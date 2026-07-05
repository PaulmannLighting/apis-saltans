use macaddr::MacAddr8;

crate::zdp_command! {
    /// IEEE Address Response.
    IeeeAddrRsp => IEEE_addr_rsp;
    cluster_id: 0x8001;
    group: DeviceAndServiceDiscovery;
    fields {
        status: u8,
        ieee_addr_remote_dev: Option<MacAddr8>,
        nwk_addr_remote_dev: Option<u16>,
        num_assoc_dev: Option<u8>,
        start_index: Option<u8>,
        nwk_addr_assoc_dev_list: Box<[u16]>,
    }
    getters {
    }
}
