//! Smarthome-facing Zigbee API facade.

use ::smarthomelib::{ZigbeeApiHandle, ZigbeeApiMessage};
use tokio::sync::mpsc;

use self::actor::ZigbeeApiActor;

mod actor;

const ZIGBEE_API_BUFFER_SIZE: usize = 32;

/// Lower-level Zigbee actor handles injected into the API actor.
#[derive(Clone, Debug, Default)]
pub struct ZigbeeApiHandles {
    // Future fields once `zigbee-coordinator` exposes these handle types:
    // network_manager: zigbee_coordinator::NetworkManagerHandle,
    // zcl_transceiver: zigbee_coordinator::ZclTransceiverHandle,
}

impl ZigbeeApiHandles {
    /// Build empty no-op handles until coordinator handle accessors are available.
    #[must_use]
    pub const fn noop() -> Self {
        Self {}
    }
}

/// Start the Zigbee API actor.
#[must_use]
pub fn start(handles: ZigbeeApiHandles) -> ZigbeeApiHandle {
    let (zigbee_api_sender, zigbee_api_mailbox) =
        mpsc::channel::<ZigbeeApiMessage>(ZIGBEE_API_BUFFER_SIZE);
    let zigbee_api_handle = ZigbeeApiHandle::new(zigbee_api_sender);
    let actor = ZigbeeApiActor::new(zigbee_api_mailbox, handles);
    tokio::spawn(actor.run());
    zigbee_api_handle
}
