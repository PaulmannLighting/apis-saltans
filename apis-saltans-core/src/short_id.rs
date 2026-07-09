pub use self::broadcast::Broadcast;
pub use self::device::Device;
use self::reserved::Reserved;

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
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(try_from = "u16", into = "u16")
)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ShortId {
    /// Coordinator short address `0x0000`.
    Coordinator,

    /// Allocated device short address in the normal device range.
    Device(Device),

    /// Broadcast short address.
    Broadcast(Broadcast),
}

impl ShortId {
    /// Return the raw 16-bit NWK short address value.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        match self {
            Self::Coordinator => COORDINATOR,
            Self::Device(device) => device.as_u16(),
            Self::Broadcast(broadcast) => broadcast.as_u16(),
        }
    }
}

impl From<Device> for ShortId {
    fn from(device: Device) -> Self {
        Self::Device(device)
    }
}

impl From<Broadcast> for ShortId {
    fn from(broadcast: Broadcast) -> Self {
        Self::Broadcast(broadcast)
    }
}

impl From<ShortId> for u16 {
    fn from(id: ShortId) -> Self {
        id.as_u16()
    }
}

impl TryFrom<u16> for ShortId {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            COORDINATOR => Ok(Self::Coordinator),
            Device::MIN_VALUE..=Device::MAX_VALUE => Ok(Device(value).into()),
            Reserved::MIN_VALUE..=Reserved::MAX_VALUE => Err(value),
            LOW_POWER_ROUTERS => Ok(Broadcast::LowPowerRouters.into()),
            ROUTERS_AND_COORDINATOR => Ok(Broadcast::RoutersAndCoordinator.into()),
            RX_ON_WHEN_IDLE => Ok(Broadcast::RxOnWhenIdle.into()),
            ALL_DEVICES => Ok(Broadcast::AllDevices.into()),
            INVALID => Err(INVALID),
        }
    }
}
