use crate::serial_port::SerialPort;
use crate::transport::FirmwareStatus;
use std::io::Read;

pub struct FirmwareUpdateHandler<'a, const BUF_SIZE: usize> {
    firmware: &'a dyn Read,
    serial_port: SerialPort<'a, BUF_SIZE>,
    callback: &'a fn(FirmwareStatus),
}
