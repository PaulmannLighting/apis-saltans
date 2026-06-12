//! Zigbee API actor implementation.

use ::smarthomelib::ZigbeeApiMessage;
use log::error;
use tokio::sync::{mpsc, oneshot};

use super::ZigbeeApiHandles;

/// Actor handling high-level Zigbee API messages.
#[derive(Debug)]
pub(super) struct ZigbeeApiActor {
    zigbee_api_mailbox: mpsc::Receiver<ZigbeeApiMessage>,
    // Future injected lower-level coordinator handles live here once exported by `zigbee-coordinator`.
    #[allow(dead_code)]
    handles: ZigbeeApiHandles,
}

impl ZigbeeApiActor {
    /// Create a Zigbee API actor around its mailbox.
    #[must_use]
    pub(super) const fn new(
        zigbee_api_mailbox: mpsc::Receiver<ZigbeeApiMessage>,
        handles: ZigbeeApiHandles,
    ) -> Self {
        Self {
            zigbee_api_mailbox,
            handles,
        }
    }

    /// Run the actor until every handle is dropped.
    pub(super) async fn run(mut self) {
        while let Some(message) = self.zigbee_api_mailbox.recv().await {
            Self::handle_message(message);
        }
    }

    fn handle_message(message: ZigbeeApiMessage) {
        match message {
            ZigbeeApiMessage::ListDevices { response } => {
                // Future: ask `handles.network_manager` for known devices and map them to records.
                send_response(response, Ok(Vec::new()));
            }
            ZigbeeApiMessage::RestoreDevices { response, .. } => {
                // Future: seed `handles.network_manager` with persisted device records.
                send_response(response, Ok(()));
            }
            ZigbeeApiMessage::AllowJoins { response, .. } => {
                // Future: request permit-join through the coordinator/network-manager handle.
                send_response(response, Ok(()));
            }
            ZigbeeApiMessage::ExecuteCommand { response, .. } => {
                // Future: resolve the endpoint through `handles.network_manager`, then send ZCL through
                // `handles.zcl_transceiver`.
                send_response(response, Ok(()));
            }
            ZigbeeApiMessage::RequestDeviceUpdate { response, .. } => {
                // Future: request rediscovery/attribute refresh through coordinator handles.
                send_response(response, Ok(()));
            }
            ZigbeeApiMessage::SubscribeDeviceEvents { events, response } => {
                // Future: register `events` with the device-discovery/network-manager event stream.
                send_response(response, Ok(()));
                drop(events);
            }
            ZigbeeApiMessage::SubscribeInteractionEvents { events, response } => {
                // Future: register `events` with protocol interaction event forwarding.
                send_response(response, Ok(()));
                drop(events);
            }
        }
    }
}

fn send_response<Response>(response: oneshot::Sender<Response>, result: Response) {
    if response.send(result).is_err() {
        error!("Zigbee runtime requester dropped response channel before response could be sent");
    }
}
