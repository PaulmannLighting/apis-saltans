use zb_aps::Data;
use zb_core::ShortId;
use zb_zcl::Cluster;
use zb_zdp::Command;

pub use self::device::Device;
pub use self::network::{Error as NetworkError, Network};

mod device;
mod network;

#[derive(Clone, Debug)]
pub enum Event {
    Network(Network),

    Device(Device),

    Zcl {
        src_address: ShortId,
        aps_frame: Data<zb_zcl::Frame<Cluster>>,
    },

    Zdp {
        src_address: ShortId,
        aps_frame: Data<zb_zdp::Frame<Command>>,
    },
}
