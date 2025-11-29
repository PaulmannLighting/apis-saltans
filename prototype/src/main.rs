//! A prototype for testing the EZSP UART implementation.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use ashv2::{BaudRate, open};
use clap::Parser;
use ezsp::Ezsp;
use ezsp::ember::concentrator;
use ezsp::ezsp::{config, decision, policy};
use ezsp::uart::Uart;
use ezsp::zigbee::{EventHandler, NetworkManager};
use log::{info, warn};
use macaddr::MacAddr8;
use serialport::FlowControl;

const PAN_ID: u16 = 24171;
const EXTENDED_PAN_ID: MacAddr8 = MacAddr8::new(0x8D, 0x9F, 0x3D, 0xFE, 0x00, 0xBF, 0x0D, 0xB5);
const RADIO_CHANNEL: u8 = 11;
const LINK_KEY: &[u8] = include_bytes!("../../assets/link.key");

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
    #[clap(long, short, help = "The PAN ID for the new network", default_value_t = PAN_ID)]
    pan_id: u16,
    #[clap(long, short, help = "The Extended PAN ID for the new network", default_value_t = EXTENDED_PAN_ID)]
    extended_pan_id: MacAddr8,
    #[clap(long, short = 'c', help = "The radio channel for the new network", default_value_t = RADIO_CHANNEL)]
    radio_channel: u8,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    env_logger::init();
    let network_up = Arc::new(AtomicBool::new(false));
    let network_open = Arc::new(AtomicBool::new(false));

    let serial_port = open(args.tty.clone(), BaudRate::RstCts, FlowControl::Software)
        .expect("Failed to open serial port");
    let (callbacks_sender, callbacks_receiver) = tokio::sync::mpsc::channel(1024);
    let (zigbee_tx, mut zigbee_rx) = tokio::sync::mpsc::channel(1024);

    let mut uart = Uart::new(serial_port, callbacks_sender, 8, 1024);
    uart.init().await.expect("Failed to initialize UART");

    let concentrator_config = concentrator::Parameters::new(
        concentrator::Type::HighRam,
        Duration::from_secs(60),
        Duration::from_secs(3600),
        8,
        8,
        0,
    )
    .expect("Concentrator parameters should be valid.");

    let mut configuration = BTreeMap::new();
    configuration.insert(config::Id::IndirectTransmissionTimeout, 7680);
    configuration.insert(config::Id::MaxHops, 30);
    configuration.insert(config::Id::PacketBufferCount, 250);
    configuration.insert(config::Id::FragmentDelayMs, 50);
    configuration.insert(config::Id::AddressTableSize, 8);
    configuration.insert(config::Id::TxPowerMode, 0);
    configuration.insert(config::Id::ApsUnicastMessageCount, 16);
    configuration.insert(config::Id::SupportedNetworks, 1);
    configuration.insert(config::Id::ApplicationZdoFlags, 1);
    configuration.insert(config::Id::TrustCenterAddressCacheSize, 2);
    configuration.insert(config::Id::StackProfile, 2);
    configuration.insert(config::Id::BroadcastTableSize, 15);
    configuration.insert(config::Id::NeighborTableSize, 24);
    configuration.insert(config::Id::MaxEndDeviceChildren, 16);
    configuration.insert(config::Id::SourceRouteTableSize, 100);
    configuration.insert(config::Id::SecurityLevel, 5);
    configuration.insert(config::Id::KeyTableSize, 4);
    configuration.insert(config::Id::FragmentWindowSize, 1);
    configuration.insert(config::Id::BindingTableSize, 2);

    let mut policy = BTreeMap::new();
    policy.insert(
        policy::Id::TcKeyRequest,
        decision::Id::AllowTcKeyRequestsAndSendCurrentKey,
    );
    policy.insert(
        policy::Id::MessageContentsInCallback,
        decision::Id::MessageTagOnlyInCallback,
    );
    policy.insert(
        policy::Id::TrustCenter,
        decision::Id::AllowPreconfiguredKeyJoins,
    );
    policy.insert(
        policy::Id::TcJoinsUsingWellKnownKey,
        decision::Id::AllowJoins,
    );
    policy.insert(
        policy::Id::BindingModification,
        decision::Id::CheckBindingModificationsAreValidEndpointClusters,
    );

    let mut network_manager = NetworkManager::new(uart, network_up.clone(), network_open.clone());
    let event_handler = EventHandler::new(callbacks_receiver, zigbee_tx, network_up, network_open);

    // Handle EZSP events.
    tokio::spawn(async move {
        event_handler.run().await;
    });

    // Handle Zigbee events.
    tokio::spawn(async move {
        while let Some(event) = zigbee_rx.recv().await {
            info!("Zigbee Event: {event:?}");
        }

        warn!("Zigbee event channel closed.");
    });

    network_manager
        .init(
            args.reinitialize,
            concentrator_config,
            configuration,
            policy,
            LINK_KEY.try_into().expect("Link key is valid."),
            args.extended_pan_id,
            args.pan_id,
            args.radio_channel,
        )
        .await
        .expect("Failed to initialize network manager");

    info!(
        "Network initialized. Permitting joining for {} seconds...",
        args.join_secs
    );

    network_manager
        .allow_joins(args.join_secs.into())
        .await
        .expect("Failed to allow joins");

    network_manager.await_network_open().await;
    network_manager.await_network_closed().await;
}
