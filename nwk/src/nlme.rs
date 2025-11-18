/// Stub device.
type Device = ();

/// Stub network parameters.
type Network = ();

/// Network layer management entity (NLME) trait.
pub trait Nlme {
    /// Configure a device on the network.
    fn configure(&mut self, device: Device) -> impl Future<Output = ()>;

    /// Start the network.
    fn start(&mut self) -> impl Future<Output = ()>;

    /// Join a network.
    fn join(&mut self, network: Network) -> impl Future<Output = ()>;

    /// Rejoin a network.
    fn rejoin(&mut self, network: Network) -> impl Future<Output = ()>;

    /// Leave the network.
    fn leave(&mut self) -> impl Future<Output = ()>;

    // TODO: Add more NLME methods as needed. Maybe split into separate traits.
}
