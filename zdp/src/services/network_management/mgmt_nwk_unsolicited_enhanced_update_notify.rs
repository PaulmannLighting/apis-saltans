crate::zdp_command! {
    /// Management Network Unsolicited Enhanced Update Notify.
    derive { Copy }
    MgmtNwkUnsolicitedEnhancedUpdateNotify => Mgmt_NWK_Unsolicited_Enhanced_Update_notify;
    cluster_id: 0x803b;
    group: NetworkManagement;
    fields {
        channel_in_use: u32,
        mac_tx_ucast_total: u16,
        mac_tx_ucast_failures: u16,
        mac_tx_ucast_retries: u16,
    }
    getters {
    }
}
