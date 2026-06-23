use macaddr::MacAddr8;
use zigbee::Endpoint;

/// Destination of a message.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Destination {
    /// And endpoint of a device.
    Endpoint {
        /// The IEEE address of the device.
        ieee_address: MacAddr8,
        /// The application endpoint on the device.
        endpoint: Endpoint,
    },

    /// Group cast.
    Group(u16),
}
