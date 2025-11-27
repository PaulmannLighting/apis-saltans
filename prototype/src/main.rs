//! A prototype for testing the EZSP UART implementation.

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::time::Duration;

use ashv2::{BaudRate, open};
use clap::Parser;
use ezsp::ember::Status;
use ezsp::ezsp::{config, policy};
use ezsp::parameters::networking::handler::Handler::StackStatus;
use ezsp::uart::Uart;
use ezsp::{Callback, Configuration, Mfglib, Networking, Utilities};
use log::{error, info};
use prototyping::Coordinator;
use serialport::FlowControl;
use tokio::sync::mpsc::Receiver;
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
    #[clap(long, short, help = "Whether to reinitialize the device")]
    reinitialize: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    env_logger::init();
    let network_up = Arc::new(AtomicBool::new(false));

    let serial_port = open(args.tty.clone(), BaudRate::RstCts, FlowControl::Software)
        .expect("Failed to open serial port");
    let (callbacks_sender, callbacks_receiver) = tokio::sync::mpsc::channel(1024);

    // Spawn a task to handle incoming callbacks.
    tokio::spawn(handle_callbacks(callbacks_receiver, network_up.clone()));

    let mut uart = Uart::new(serial_port, callbacks_sender, 8, 1024);

    info!("Initializing");
    uart.initialize().await.expect("Failed to initialize uart");

    info!("Starting");
    uart.startup(args.reinitialize)
        .await
        .expect("Failed to start uart");

    while !network_up.load(SeqCst) {
        info!("Waiting for network to come up!");
        sleep(Duration::from_secs(1)).await;
    }

    info!("Network is up!");

    info!("Permitting joining for {} seconds", args.join_secs);
    Coordinator::permit_joining(&mut uart, args.join_secs)
        .await
        .expect("Failed to permit joining");

    info!("Advertising network for {} seconds", args.join_secs);
    uart.advertise_network(args.join_secs)
        .await
        .expect("Failed to advertise network");

    sleep(Duration::from_secs(args.join_secs.into())).await;
    info!("Joining period has ended");

    info!("Listing children");
    let _children = uart.get_children().await;

    match uart.get_policy(policy::Id::TrustCenter).await {
        Ok(policy) => info!("Trust center policy: {policy:?}"),
        Err(error) => error!("Failed to get trust center policy: {error}"),
    }

    match uart.get_power().await {
        Ok(radio_power) => info!("Radio power: {radio_power} dBm"),
        Err(error) => error!("Failed to get radio power: {error}"),
    }

    match uart
        .get_configuration_value(config::Id::NeighborTableSize)
        .await
    {
        Ok(size) => info!("Neighbor table size: {size}"),
        Err(error) => error!("Failed to get neighbor table size: {error}"),
    }

    match uart
        .get_configuration_value(config::Id::SourceRouteTableSize)
        .await
    {
        Ok(size) => info!("Source route table size: {size}"),
        Err(error) => error!("Failed to get source route table size: {error}"),
    }

    match uart.get_radio_channel().await {
        Ok(channel) => info!("Radio channel: {channel}"),
        Err(error) => error!("Failed to get radio channel: {error}"),
    }

    while let Some(callback) = uart.callback().await.expect("Failed to receive callback") {
        info!("Received callback: {callback:?}");
    }

    info!("Terminating.");
}

async fn handle_callbacks(mut callbacks: Receiver<Callback>, network_up: Arc<AtomicBool>) {
    while let Some(callback) = callbacks.recv().await {
        info!("Received callback: {callback:?}");

        if let Callback::Networking(StackStatus(status)) = &callback
            && status.result() == Ok(Status::NetworkUp)
        {
            network_up.store(true, SeqCst);
        };
    }
}
