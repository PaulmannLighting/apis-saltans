//! A prototype for testing the EZSP UART implementation.

use std::time::Duration;

use ashv2::{BaudRate, open};
use clap::Parser;
use ezsp::uart::Uart;
use log::info;
use prototyping::Coordinator;
use serialport::FlowControl;
use tokio::time::sleep;

#[derive(Debug, Parser)]
struct Args {
    #[clap(index = 1, help = "Path to the serial TTY device")]
    tty: String,
    #[clap(
        long,
        short,
        default_value_t = 60,
        help = "Amount of seconds to permit joining"
    )]
    join_secs: u8,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    env_logger::init();

    let serial_port = open(args.tty.clone(), BaudRate::RstCts, FlowControl::Software)
        .expect("Failed to open serial port");
    let (callbacks_sender, mut callbacks_receiver) = tokio::sync::mpsc::channel(1024);

    tokio::spawn(async move {
        while let Some(callback) = callbacks_receiver.recv().await {
            info!("Received callback: {callback:?}");
        }
    });

    let mut uart = Uart::new(serial_port, callbacks_sender, 8, 1024);
    uart.initialize().await.expect("Failed to initialize uart");
    uart.form_network(0xabcd, 11)
        .await
        .expect("Failed to form network");

    uart.permit_joining(args.join_secs)
        .await
        .expect("Failed to permit joining");
    sleep(Duration::from_secs(args.join_secs.into())).await;
}
