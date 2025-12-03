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
use ezsp::zigbee::NetworkManager;
use log::info;
use macaddr::MacAddr8;
use rocket::routes;
use serialport::FlowControl;
use tokio::sync::Mutex;

use crate::web_api::{allow_join, get_neighbors, set_color, switch_off, switch_on};

const PAN_ID: u16 = 24171;
const EXTENDED_PAN_ID: MacAddr8 = MacAddr8::new(0x8D, 0x9F, 0x3D, 0xFE, 0x00, 0xBF, 0x0D, 0xB5);
const RADIO_CHANNEL: u8 = 11;
const RADIO_TX_POWER: i8 = 8;
const LINK_KEY: &[u8] = include_bytes!("../../assets/link.key");
const NETWORK_KEY: [u8; 16] = [
    0x29, 0xB0, 0x0D, 0xE6, 0x31, 0xAB, 0x7A, 0xD0, 0xC6, 0x83, 0xC8, 0x7A, 0xBF, 0x70, 0xD6, 0x08,
];

#[derive(Debug, Parser)]
struct Args {
    #[clap(index = 1, help = "Path to the serial TTY device")]
    tty: String,
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
    let (callbacks_tx, callbacks_rx) = tokio::sync::mpsc::channel(1024);

    let mut uart = Uart::new(serial_port, callbacks_tx, 8, 1024);
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

    let network_manager = NetworkManager::build(uart, callbacks_rx)
        .with_policies(policy)
        .with_configurations(configuration)
        .with_concentrator(concentrator_config)
        .with_link_key(LINK_KEY.try_into().expect("Link key is valid."))
        .with_network_key(NETWORK_KEY)
        .with_ieee_address(args.extended_pan_id)
        .with_pan_id(args.pan_id)
        .with_radio_channel(args.radio_channel)
        .with_radio_power(RADIO_TX_POWER)
        .with_reinitialize(args.reinitialize)
        .start()
        .await
        .expect("Failed to start network manager");

    let figment = rocket::Config::figment().merge(("address", Ipv4Addr::UNSPECIFIED));
    info!("Starting server...");
    let _web_ui = rocket::custom(figment)
        .manage(Arc::new(Mutex::new(network_manager)))
        .mount(
            "/",
            routes![allow_join, get_neighbors, switch_on, switch_off, set_color],
        )
        .launch()
        .await
        .expect("Failed to launch server");
    info!("Server stopped.");
}
