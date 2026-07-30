use zb_aps::apsde::DataIndication;
use zb_zcl::{Cluster, Frame};

pub use self::device::Device;
pub use self::network::{Error as NetworkError, Network};

mod device;
mod network;

/// Event emitted by the coordinator runtime.
#[derive(Clone, Debug)]
pub enum Event {
    /// Network-level state or error notification.
    Network(Network),

    /// Device lifecycle or activity notification.
    Device(Device),

    /// Unmatched inbound ZCL indication.
    Zcl {
        /// Normalized APSDE indication containing the parsed ZCL frame and receive metadata.
        ///
        /// Backend-specific receive timestamps and device-key-pair handles are normalized to
        /// `()`. Addressing, profile, cluster, status, security, and link-quality metadata remain
        /// attached.
        indication: DataIndication<Frame<Cluster>, (), ()>,
    },
}
