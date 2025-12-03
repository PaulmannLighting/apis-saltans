/// Zigbee broadcast addresses.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum Broadcast {
    /// All devices, including end devices, on the network.
    Universal = 0xFFFF,
    /// All routers and coordinators on the network.
    RoutersAndCoordinators = 0xFFFE,
    /// All devices which are not asleep.
    ActiveDevices = 0xFFFD,
}
