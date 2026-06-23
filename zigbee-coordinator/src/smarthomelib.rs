#![cfg(feature = "smarthomelib")]
//! Smart Home library integration.
//!
//! The implementations are in the respective submodules.

use std::collections::BTreeSet;

use macaddr::MacAddr8;
use smarthomelib::{Events, Protocol};
use zigbee::Endpoint;

use crate::{Coordinator, EVENT_CHANNEL_SIZE, Error, NetworkManager};

mod color_control;
mod event;
mod event_receiver;
mod groups;
mod inventory;
mod joining;
mod level_control;
mod on_off;
mod zigbee_id_codec;

impl Protocol for Coordinator {
    type DeviceId = MacAddr8;
    type EndpointId = Endpoint;
    type Error = Error;
}

impl Events for Coordinator {
    type Receiver = crate::EventReceiver;

    async fn events(&self) -> Result<Self::Receiver, Self::Error> {
        self.subscribe_to_incoming_commands(BTreeSet::new(), EVENT_CHANNEL_SIZE)
            .await
            .map(Into::into)
    }
}
