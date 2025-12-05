use std::collections::BTreeMap;
use std::time::Duration;

use macaddr::MacAddr8;
use tokio::sync::oneshot::Sender;
use zigbee::Endpoint;

use crate::{Error, Frame};

/// Messages sent to the NWK actor.
pub enum Message {
    GetTransactionSeq {
        response: Sender<u8>,
    },
    GetPanId {
        response: Sender<Result<u16, Error>>,
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
        cluster_id: u16,
        group_id: u16,
        frame: Frame,
        response: Sender<Result<(), Error>>,
    },
}
