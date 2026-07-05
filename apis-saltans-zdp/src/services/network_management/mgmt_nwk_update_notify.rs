use crate::ByteSizedVec;

crate::zdp_command! {
    /// Management Network Update Notify.
    MgmtNwkUpdateNotify => Mgmt_NWK_Update_notify;
    cluster_id: 0x8038;
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
