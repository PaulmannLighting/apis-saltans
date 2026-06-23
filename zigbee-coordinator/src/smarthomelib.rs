#![cfg(feature = "smarthomelib")]
//! Smart Home library integration.
//!
//! The implementations are in the respective submodules.

use std::collections::BTreeSet;

use macaddr::MacAddr8;
use smarthomelib::{Events, Protocol};
use zigbee::Application;

use crate::{Coordinator, EVENT_CHANNEL_SIZE, Error, NetworkManager};

mod color_control;
mod destination;
mod event;
mod event_receiver;
mod on_off;
mod translate_device_id;
mod translate_endpoint_id;

impl Protocol for Coordinator {
    type DeviceId = MacAddr8;
    type EndpointId = Application;
    type GroupId = u16;
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
