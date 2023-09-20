use crate::flow_control::FlowControl;

pub struct SerialPort<const BUF_SIZE: usize> {
    serial_port: dyn serialport::SerialPort,
    port_name: String,
    baud_rate: u32,
    flow_control: FlowControl,
    buffer: [u8; BUF_SIZE],
}
