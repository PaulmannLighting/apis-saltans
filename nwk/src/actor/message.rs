use std::collections::BTreeMap;
use std::time::Duration;

use macaddr::MacAddr8;
use tokio::sync::oneshot::Sender;
use zigbee::Endpoint;

use super::ZclCommand;
use crate::Error;

/// Messages sent to the NWK actor.
pub enum Message<E> {
    AllowJoins {
        duration: Duration,
        response: Sender<Result<(), Error<E>>>,
    },
    GetNeighbors {
        response: Sender<Result<BTreeMap<MacAddr8, u16>, Error<E>>>,
    },
    ZclCommand {
        pan_id: u16,
        endpoint: Endpoint,
        command: ZclCommand,
        response: Sender<Result<(), Error<E>>>,
    },
}
