mod firmware_update_handler;
mod frame_handler;
mod protocol;
mod serial_protocol;

use crate::security::Key;
use crate::serial_port::SerialPort;
use crate::transport::{DeviceType, Receive};
use firmware_update_handler::FirmwareUpdateHandler;
use frame_handler::FrameHandler;
use protocol::{Config, Decision, NetworkParameters, Policy};
use serial_protocol::SerialProtocol;
use std::collections::BTreeMap;

pub struct EmberZNetSerialProtocol<'a, const BUF_SIZE: usize> {
    serial_port: SerialPort<'a, BUF_SIZE>,
    frame_handler: &'a dyn FrameHandler,
    firmware_update_handler: FirmwareUpdateHandler<'a, BUF_SIZE>,
    stack_configuration: BTreeMap<Config, u8>,
    stack_policy: BTreeMap<Policy, Decision>,
    transport_receive: &'a dyn Receive,
    link_key: Key,
    network_key: Key,
    network_parameters: NetworkParameters,
    ieee_address: u64,
    network_address: u16,
    device_type: DeviceType,
    protocol: SerialProtocol,
}
