#![cfg(feature = "smarthomelib")]
//! SmartHomeLib integration.
//!
//! The implementations are in the respective submodules.

use macaddr::MacAddr8;
use smarthomelib::Protocol;
use zigbee::Endpoint;

use crate::{Coordinator, Error};

mod color_control;
mod event_receiver;
mod on_off;
mod translate_device_id;
mod translate_endpoint_id;

impl Protocol for Coordinator {
    type DeviceId = MacAddr8;
    type EndpointId = Endpoint;
    type Error = Error;
}
