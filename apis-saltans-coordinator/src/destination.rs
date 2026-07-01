use apis_saltans_core::Application;
use macaddr::MacAddr8;

/// Destination of a message.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Destination {
    /// And endpoint of a device.
    Endpoint {
        /// The IEEE address of the device.
        ieee_address: MacAddr8,
        /// The application endpoint on the device.
        endpoint: Application,
    },

    /// Group cast.
    Group(u16),
}
