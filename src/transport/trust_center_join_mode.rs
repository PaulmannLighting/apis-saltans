/// An enumeration with the Trust Center join mode.
pub enum TrustCenterJoinMode {
    /// The TC should deny joins. Even if a router allows a device to join the network,
    /// the trust center will not support the request, and will not deliver the network
    /// key to the device.
    Deny,
    /// The TC should allow joins using the link key, or a preconfigured
    /// link key / device specific install code.
    Insecure,
    /// The TC should allow joins only with a preconfigured
    /// link key / device specific install code.
    Secure,
    /// The TC should only allow devices to join when there is
    /// a device specific install code derived link key
    InstallCode,
}
