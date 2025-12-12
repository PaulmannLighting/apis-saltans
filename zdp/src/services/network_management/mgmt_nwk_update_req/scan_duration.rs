use repr_discriminant::ReprDiscriminant;

/// Scan Duration enumeration.
///
/// For `Zero` to `Five`, the duration is calculated as `<const> * (2^n)` symbols.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, ReprDiscriminant)]
#[repr(u8)]
pub enum ScanDuration {
    /// Scan duration of `<const> * (2^0)` symbols.
    Zero { energy_scan: bool } = 0x00,
    /// Scan duration of `<const> * (2^1)` symbols.
    One { energy_scan: bool } = 0x01,
    /// Scan duration of `<const> * (2^2)` symbols.
    Two { energy_scan: bool } = 0x02,
    /// Scan duration of `<const> * (2^3)` symbols.
    Three { energy_scan: bool } = 0x03,
    /// Scan duration of `<const> * (2^4)` symbols.
    Four { energy_scan: bool } = 0x04,
    /// Scan duration of `<const> * (2^5)` symbols.
    Five { energy_scan: bool } = 0x05,
    /// Channel change request.
    ChannelChange { nwk_update_id: u8 } = 0xFE,
    /// `aps_channel_mask_list` and `nwk_manager_addr` attribute change request.
    AttributeChange {
        nwk_update_id: u8,
        nwk_manager_addr: u16,
    } = 0xFF,
}
