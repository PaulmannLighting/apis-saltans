pub use self::apsde::ApsdeEvent;
pub use self::device::DeviceEvent;
pub use self::network::NetworkEvent;
pub use self::route_error::RouteError;

mod apsde;
mod device;
mod network;
mod route_error;

/// Events emitted by the hardware layer.
///
/// `T` and `K` retain the backend-defined APSDE timestamp and device-key-pair handle types. They
/// are used only by the [`ApsdeEvent`] variant.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Event<T = (), K = ()> {
    /// Network state or routing event.
    Network(NetworkEvent),

    /// Device membership event.
    Device(DeviceEvent),

    /// APS data-service indication or confirmation.
    Apsde(ApsdeEvent<T, K>),
}

impl<T, K> From<NetworkEvent> for Event<T, K> {
    fn from(event: NetworkEvent) -> Self {
        Self::Network(event)
    }
}

impl<T, K> From<DeviceEvent> for Event<T, K> {
    fn from(event: DeviceEvent) -> Self {
        Self::Device(event)
    }
}

impl<T, K> From<ApsdeEvent<T, K>> for Event<T, K> {
    fn from(event: ApsdeEvent<T, K>) -> Self {
        Self::Apsde(event)
    }
}
