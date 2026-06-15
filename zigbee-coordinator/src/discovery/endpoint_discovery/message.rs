use zigbee::Address;

/// Message sent to the endpoint discovery actor.
#[derive(Debug)]
pub enum Message {
    /// Discover endpoints on the given device.
    Discover(Address),
}
