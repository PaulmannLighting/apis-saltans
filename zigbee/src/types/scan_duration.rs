use num_derive::FromPrimitive;

/// Scan Duration enumeration.
///
/// For `Zero` to `Five`, the duration is calculated as `<const> * (2^n)` symbols.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromPrimitive)]
#[repr(u8)]
pub enum ScanDuration {
    /// Scan duration of `<const> * (2^0)` symbols.
    Zero = 0x00,
    /// Scan duration of `<const> * (2^1)` symbols.
    One = 0x01,
    /// Scan duration of `<const> * (2^2)` symbols.
    Two = 0x02,
    /// Scan duration of `<const> * (2^3)` symbols.
    Three = 0x03,
    /// Scan duration of `<const> * (2^4)` symbols.
    Four = 0x04,
    /// Scan duration of `<const> * (2^5)` symbols.
    Five = 0x05,
    /// Channel change request.
    ChannelChange = 0xFE,
    /// `aps_channel_mask_list` and `nwk_manager_addr` attribute change request.
    AttributeChange = 0xFF,
}
