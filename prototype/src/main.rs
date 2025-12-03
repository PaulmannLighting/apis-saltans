//! A prototype for testing the EZSP UART implementation.

mod web_api;

use std::collections::BTreeMap;
use std::net::Ipv4Addr;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use ashv2::{BaudRate, open};
use clap::Parser;
use ezsp::Ezsp;
use ezsp::ember::concentrator;
use ezsp::ezsp::{config, decision, policy};
use ezsp::uart::Uart;
use ezsp::zigbee::{DeviceConfig, EventHandler, NetworkManager};
use log::{debug, info};
use macaddr::MacAddr8;
use rocket::routes;
use serialport::FlowControl;
use tokio::sync::Mutex;
use zigbee_nwk::Nlme;

use crate::web_api::{allow_join, get_neighbors, set_color};

const PAN_ID: u16 = 24171;
const EXTENDED_PAN_ID: MacAddr8 = MacAddr8::new(0x8D, 0x9F, 0x3D, 0xFE, 0x00, 0xBF, 0x0D, 0xB5);
const RADIO_CHANNEL: u8 = 11;
const RADIO_TX_POWER: i8 = 8;
const LINK_KEY: &[u8] = include_bytes!("../../assets/link.key");
const NETWORK_KEY: [u8; 16] = [
    0x29, 0xB0, 0x0D, 0xE6, 0x31, 0xAB, 0x7A, 0xD0, 0xC6, 0x83, 0xC8, 0x7A, 0xBF, 0x70, 0xD6, 0x08,
];
const HOME_AUTOMATION: u16 = 0x0104;

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
    #[clap(long, short, help = "The Extended PAN ID for the new network", default_value_t = EXTENDED_PAN_ID
    )]
    extended_pan_id: MacAddr8,
    #[clap(long, short = 'c', help = "The radio channel for the new network", default_value_t = RADIO_CHANNEL
    )]
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
    configuration.insert(config::Id::SourceRouteTableSize, 16);
    configuration.insert(config::Id::SecurityLevel, 5);
    configuration.insert(config::Id::AddressTableSize, 8);
    configuration.insert(config::Id::TrustCenterAddressCacheSize, 2);
    configuration.insert(config::Id::StackProfile, 2);
    configuration.insert(config::Id::IndirectTransmissionTimeout, 7680);
    configuration.insert(config::Id::MaxHops, 8);
    configuration.insert(config::Id::TxPowerMode, 0);
    configuration.insert(config::Id::SupportedNetworks, 1);
    configuration.insert(config::Id::KeyTableSize, 4);
    configuration.insert(config::Id::ApplicationZdoFlags, 1);
    configuration.insert(config::Id::MaxEndDeviceChildren, 16);
    configuration.insert(config::Id::ApsUnicastMessageCount, 10);
    configuration.insert(config::Id::BroadcastTableSize, 15);
    configuration.insert(config::Id::BindingTableSize, 2);
    configuration.insert(config::Id::NeighborTableSize, 16);
    configuration.insert(config::Id::FragmentWindowSize, 1);
    configuration.insert(config::Id::FragmentDelayMs, 50);
    configuration.insert(config::Id::PacketBufferCount, 255);

    let mut policy = BTreeMap::new();
    policy.insert(
        policy::Id::TrustCenter,
        decision::Id::AllowPreconfiguredKeyJoins.into(),
    );
    policy.insert(
        policy::Id::TcJoinsUsingWellKnownKey,
        decision::Id::AllowJoins.into(),
    );
    policy.insert(
        policy::Id::TcKeyRequest,
        decision::Id::AllowTcKeyRequestsAndSendCurrentKey.into(),
    );
    policy.insert(
        policy::Id::MessageContentsInCallback,
        decision::Id::MessageTagOnlyInCallback.into(),
    );
    policy.insert(
        policy::Id::BindingModification,
        decision::Id::CheckBindingModificationsAreValidEndpointClusters.into(),
    );
    policy.insert(
        policy::Id::KeyRequest,
        (decision::Bitmask::ALLOW_JOINS | decision::Bitmask::IGNORE_UNSECURED_REJOINS).bits(),
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

        debug!("Zigbee event channel closed.");
    });

    let device_config = DeviceConfig::new(
        concentrator_config,
        configuration,
        policy,
        LINK_KEY.try_into().expect("Link key is valid."),
        NETWORK_KEY,
        args.extended_pan_id,
        args.pan_id,
        args.radio_channel,
        RADIO_TX_POWER,
    );

    network_manager
        .configure(device_config)
        .await
        .expect("Failed to initialize network manager");
    network_manager
        .start(args.reinitialize)
        .await
        .expect("Failed to start network manager");

    let figment = rocket::Config::figment().merge(("address", Ipv4Addr::UNSPECIFIED));
    info!("Starting server...");
    let _web_ui = rocket::custom(figment)
        .manage(Arc::new(Mutex::new(network_manager)))
        .mount("/", routes![allow_join, get_neighbors, set_color])
        .launch()
        .await
        .expect("Failed to launch server");
    info!("Server stopped.");
}
