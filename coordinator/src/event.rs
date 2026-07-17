use zb_aps::Data;
use zb_core::ShortId;
use zb_zcl::Cluster;

pub use self::device::Device;
pub use self::network::{Error as NetworkError, Network};

mod device;
mod network;

/// Event emitted by the coordinator runtime.
#[derive(Clone, Debug)]
pub enum Event {
    /// Network-level state or error notification.
    Network(Network),

    /// Device lifecycle notification.
    Device(Device),

    /// Unmatched inbound ZCL frame.
    Zcl {
        /// NWK short address of the sender.
        src_address: ShortId,

        /// Received APS frame containing the parsed ZCL frame.
        aps_frame: Data<zb_zcl::Frame<Cluster>>,
    },
}
