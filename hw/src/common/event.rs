pub use self::aps::ApsEvent;
pub use self::device::DeviceEvent;
pub use self::network::NetworkEvent;
pub use self::route_error::RouteError;

mod aps;
mod device;
mod network;
mod route_error;

/// Events emitted by the hardware layer.
#[derive(Clone, Debug)]
pub enum Event {
    /// Network state or routing event.
    Network(NetworkEvent),

    /// Device membership event.
    Device(DeviceEvent),

    /// APS receive or transmission event.
    Aps(ApsEvent),
}
