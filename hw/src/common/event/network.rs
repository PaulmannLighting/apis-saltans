use super::RouteError;

/// Network events emitted by the hardware layer.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum NetworkEvent {
    /// The network is up and running.
    Up,

    /// The network is down.
    Down,

    /// The network has been opened for new joins.
    Opened,

    /// The network has been closed for new joins.
    Closed,

    /// A routing error occurred.
    RouteError(RouteError),
}
