use crate::ByteSizedVec;

crate::zdp_command! {
    /// Management Network Enhanced Update Notify.
    MgmtNwkEnhancedUpdateNotify => Mgmt_NWK_Enhanced_Update_notify;
    cluster_id: 0x8039;
    group: NetworkManagement;
    fields {
        status: u8,
        scanned_channels: u32,
        total_transmissions: u16,
        transmission_failures: u16,
        energy_values: ByteSizedVec<u8>,
    }
    getters {
    }
}
