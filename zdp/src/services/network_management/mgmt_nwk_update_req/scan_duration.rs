use repr_discriminant::ReprDiscriminant;

/// Scan Duration enumeration.
///
/// For `Zero` to `Five`, the duration is calculated as `<const> * (2^n)` symbols.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, ReprDiscriminant)]
#[repr(u8)]
pub enum ScanDuration {
    /// Scan duration of `<const> * (2^0)` symbols.
    Zero {
        /// Perform an energy scan if `true`.
        energy_scan: bool,
    } = 0x00,
    /// Scan duration of `<const> * (2^1)` symbols.
    One {
        /// Perform an energy scan if `true`.
        energy_scan: bool,
    } = 0x01,
    /// Scan duration of `<const> * (2^2)` symbols.
    Two {
        /// Perform an energy scan if `true`.
        energy_scan: bool,
    } = 0x02,
    /// Scan duration of `<const> * (2^3)` symbols.
    Three {
        /// Perform an energy scan if `true`.
        energy_scan: bool,
    } = 0x03,
    /// Scan duration of `<const> * (2^4)` symbols.
    Four {
        /// Perform an energy scan if `true`.
        energy_scan: bool,
    } = 0x04,
    /// Scan duration of `<const> * (2^5)` symbols.
    Five {
        /// Perform an energy scan if `true`.
        energy_scan: bool,
    } = 0x05,
    /// Channel change request.
    ChannelChange {
        /// Network update ID.
        nwk_update_id: u8,
    } = 0xFE,
    /// `aps_channel_mask_list` and `nwk_manager_addr` attribute change request.
    AttributeChange {
        /// Network update ID.
        nwk_update_id: u8,
        /// Network manager address.
        nwk_manager_addr: u16,
    } = 0xFF,
}
