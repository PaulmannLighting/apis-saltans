mod serial_protocol;

use crate::security::Key;
use crate::serial_port::SerialPort;
use crate::transport::DeviceType;
use serial_protocol::SerialProtocol;
use std::collections::BTreeMap;

pub struct EmberZNetSerialProtocol<'serial_port, const BUF_SIZE: usize> {
    serial_port: SerialPort<'serial_port, BUF_SIZE>,
    frame_handler: FrameHandler,
    firmware_handler: FirmwareHandler,
    stack_configuration: BTreeMap<ConfigId, u8>,
    stack_policy: BTreeMap<PolicyId, DecisionId>,
    transport_receive: TransportReceive,
    link_key: Key,
    network_key: Key,
    network_parameters: NetworkParameters,
    ieee_address: u64,
    network_address: u16,
    device_type: DeviceType,
    protocol: SerialProtocol,
}
