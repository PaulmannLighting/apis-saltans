use std::collections::BTreeMap;
use std::time::Duration;

use macaddr::MacAddr8;
use tokio::sync::oneshot::Sender;
use zigbee::Endpoint;

pub use self::found_network::{FoundNetwork, Network};
use crate::{Error, Frame};

mod found_network;

/// Messages sent to the NWK actor.
pub enum Message {
    GetTransactionSeq {
        response: Sender<u8>,
    },
    GetPanId {
        response: Sender<Result<u16, Error>>,
    },
    ScanNetworks {
        response: Sender<Result<Vec<FoundNetwork>, Error>>,
    },
    AllowJoins {
        duration: Duration,
        response: Sender<Result<(), Error>>,
    },
    GetNeighbors {
        response: Sender<Result<BTreeMap<MacAddr8, u16>, Error>>,
    },
    Unicast {
        pan_id: u16,
        endpoint: Endpoint,
        frame: Frame,
        response: Sender<Result<(), Error>>,
    },
}
