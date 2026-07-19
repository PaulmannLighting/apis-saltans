use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;
use zb_hw::RouteError;

/// Network state event.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Network {
    /// The network is up.
    Up,

    /// The network is down.
    Down,

    /// Joining has been opened.
    Opened,

    /// Joining has been closed.
    Closed,

    /// A network-level error occurred.
    Error(Error),
}

/// Network-level error event.
#[derive(
    Clone, Copy, Debug, Eq, ThisError, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum Error {
    /// Route error reported by the hardware layer.
    #[error("{0}")]
    Route(#[from] RouteError),
}
