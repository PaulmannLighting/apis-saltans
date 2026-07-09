pub use self::broadcast::Broadcast;
pub use self::device::Device;
pub use self::reserved::Reserved;

mod broadcast;
mod device;
mod reserved;

const COORDINATOR: u16 = 0x0000;
const LOW_POWER_ROUTERS: u16 = Broadcast::LowPowerRouters.as_u16();
const ROUTERS_AND_COORDINATOR: u16 = Broadcast::RoutersAndCoordinator.as_u16();
const RX_ON_WHEN_IDLE: u16 = Broadcast::RxOnWhenIdle.as_u16();
const ALL_DEVICES: u16 = Broadcast::AllDevices.as_u16();
const INVALID: u16 = 0xFFFE;

/// Zigbee NWK short address.
///
/// This type separates ordinary device addresses from Zigbee's special short
/// address values, including the coordinator address, reserved range, broadcast
/// destinations, and the `0xFFFE` invalid or unknown-address sentinel.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ShortId {
    /// Coordinator short address `0x0000`.
    Coordinator,

    /// Allocated device short address in the normal device range.
    Device(Device),

    /// Reserved short address.
    Reserved(Reserved),

    /// Broadcast short address.
    Broadcast(Broadcast),

    /// Invalid, unknown, or unassigned short address sentinel `0xFFFE`.
    Invalid,
}

impl ShortId {
    /// Return the raw 16-bit NWK short address value.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        match self {
            Self::Coordinator => COORDINATOR,
            Self::Device(device) => device.as_u16(),
            Self::Reserved(reserved) => reserved.as_u16(),
            Self::Broadcast(broadcast) => broadcast.as_u16(),
            Self::Invalid => INVALID,
        }
    }
}

impl From<Device> for ShortId {
    fn from(device: Device) -> Self {
        Self::Device(device)
    }
}

impl From<Reserved> for ShortId {
    fn from(reserved: Reserved) -> Self {
        Self::Reserved(reserved)
    }
}

impl From<Broadcast> for ShortId {
    fn from(broadcast: Broadcast) -> Self {
        Self::Broadcast(broadcast)
    }
}

impl From<u16> for ShortId {
    fn from(value: u16) -> Self {
        match value {
            COORDINATOR => Self::Coordinator,
            Device::MIN_VALUE..=Device::MAX_VALUE => Device(value).into(),
            Reserved::MIN_VALUE..=Reserved::MAX_VALUE => Reserved(value).into(),
            LOW_POWER_ROUTERS => Broadcast::LowPowerRouters.into(),
            ROUTERS_AND_COORDINATOR => Broadcast::RoutersAndCoordinator.into(),
            RX_ON_WHEN_IDLE => Broadcast::RxOnWhenIdle.into(),
            ALL_DEVICES => Broadcast::AllDevices.into(),
            INVALID => Self::Invalid,
        }
    }
}

impl From<ShortId> for u16 {
    fn from(id: ShortId) -> Self {
        id.as_u16()
    }
}
