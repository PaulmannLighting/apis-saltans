use std::collections::BTreeMap;
use std::time::Duration;

use macaddr::MacAddr8;
use tokio::sync::oneshot::Sender;
use zcl::Commands;
use zigbee::Endpoint;

use crate::Error;

/// Messages sent to the NWK actor.
#[expect(clippy::large_enum_variant, variant_size_differences)]
pub enum Message {
    AllowJoins {
        duration: Duration,
        response: Sender<Result<(), Error>>,
    },
    GetNeighbors {
        response: Sender<Result<BTreeMap<MacAddr8, u16>, Error>>,
    },
    ZclCommand {
        pan_id: u16,
        endpoint: Endpoint,
        command: Commands,
        response: Sender<Result<(), Error>>,
    },
}
