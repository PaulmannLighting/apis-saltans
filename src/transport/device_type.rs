/// Defines the configuration options for [`TransportConfigOption::DEVICE_TYPE`].
pub enum DeviceType {
    /// ZigBee Coordinator (ZC)
    ///
    /// Will relay messages and can act as a parent to other nodes.
    Coordinator,
    /// ZigBee Router (ZR)
    ///
    /// Will relay messages and can act as a parent to other nodes.
    Router,
    /// ZigBee End Device (ZED)
    ///
    /// Communicates only with its parent and will not relay messages.
    EndDevice,
    /// ZigBee Sleepy End Device (ZSED)
    ///
    /// An end device whose radio can be turned off to save power. The application must poll to receive messages.
    SleepyEndDevice,
}
