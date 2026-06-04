/// The network management actor.
pub struct Actor {}

/// Messages received by the network management actor.
#[derive(Debug)]
pub enum Message {
    /// The network is up.
    NetworkUp,
    /// The network is down.
    NetworkDown,
    /// The network is open for new devices to join.
    NetworkOpened,
    /// The network has been closed.
    NetworkClosed,
}
