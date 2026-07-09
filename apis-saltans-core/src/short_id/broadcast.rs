/// Zigbee NWK broadcast short address.
///
/// Broadcast addresses occupy the high end of the 16-bit NWK short-address
/// space and select different receiver sets.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
pub enum Broadcast {
    /// All devices in the network.
    AllDevices = 0xFFFF,

    /// Devices with receivers enabled while idle.
    RxOnWhenIdle = 0xFFFD,

    /// Routers and the coordinator.
    RoutersAndCoordinator = 0xFFFC,

    /// Low-power routers, if supported by the Zigbee profile/revision.
    LowPowerRouters = 0xFFFB,
}

impl Broadcast {
    /// Return the raw 16-bit NWK broadcast address.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self as u16
    }
}

impl From<Broadcast> for u16 {
    fn from(broadcast: Broadcast) -> Self {
        broadcast.as_u16()
    }
}
