use std::time::Instant;

use zb_aps::apsde::DataIndication;
use zb_zcl::{Cluster, Frame};

pub use self::device::{Device, KeepAlive};
pub use self::network::{Error as NetworkError, Network};
pub use self::sink::{DroppedEvents, EventSink};

mod device;
mod network;
mod sink;

/// An APSDE indication with a host monotonic timestamp captured at coordinator ingress.
///
/// Read the timestamp through `metadata().rx_time()`. It records when the coordinator accepts
/// the hardware indication, before parsing or forwarding it. The device-key-pair handle uses
/// the default unit type.
pub type ReceivedIndication<A> = DataIndication<A, Instant>;

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
        /// The APS metadata includes the coordinator ingress timestamp.
        indication: ReceivedIndication<Frame<Cluster>>,
    },
}
