//! A prototype for testing the EZSP UART implementation.

use ashv2::{BaudRate, open};
use clap::Parser;
use ezsp::uart::Uart;
use prototyping::Coordinator;
use serialport::FlowControl;

#[derive(Debug, Parser)]
struct Args {
    tty: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    env_logger::init();

    let serial_port = open(args.tty.clone(), BaudRate::RstCts, FlowControl::Software)
        .expect("Failed to open serial port");
    let (callbacks_sender, _callbacks_receiver) = tokio::sync::mpsc::channel(1024);

    let mut uart = Uart::new(serial_port, callbacks_sender, 8, 1024);
    uart.initialize().await.expect("Failed to initialize uart");
    uart.form_network(0xabcd, 11)
        .await
        .expect("Failed to form network");
}
