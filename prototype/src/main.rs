//! A prototype for testing the EZSP UART implementation.

use std::collections::BTreeMap;
use std::io::stdin;
use std::panic::set_hook;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use ashv2::{BaudRate, HexSlice, open};
use clap::Parser;
use ezsp::ember::message::Destination;
use ezsp::ember::{aps, concentrator};
use ezsp::ezsp::{config, decision, policy};
use ezsp::uart::Uart;
use ezsp::zigbee::{DeviceConfig, EventHandler, NetworkManager};
use ezsp::{Ezsp, Messaging, Networking};
use le_stream::ToLeStream;
use log::{debug, error, info};
use macaddr::MacAddr8;
use serialport::FlowControl;
use tokio::time::sleep;
use zcl::Cluster;
use zcl::general::identify::{EffectIdentifier, EffectVariant, TriggerEffect};
use zcl::general::on_off::Off;
use zcl::lighting::color_control::MoveToColor;
use zigbee_nwk::Nlme;

const PAN_ID: u16 = 24171;
const EXTENDED_PAN_ID: MacAddr8 = MacAddr8::new(0x8D, 0x9F, 0x3D, 0xFE, 0x00, 0xBF, 0x0D, 0xB5);
const RADIO_CHANNEL: u8 = 11;
const LINK_KEY: &[u8] = include_bytes!("../../assets/link.key");
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
        policy::Id::TcKeyRequest,
        decision::Id::AllowTcKeyRequestsAndSendCurrentKey,
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
        policy::Id::MessageContentsInCallback,
        decision::Id::MessageTagOnlyInCallback,
    );
    policy.insert(policy::Id::KeyRequest, decision::Id::DenyAppKeyRequests);
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

        debug!("Zigbee event channel closed.");
    });

    let device_config = DeviceConfig::new(
        concentrator_config,
        configuration,
        policy,
        LINK_KEY.try_into().expect("Link key is valid."),
        args.extended_pan_id,
        args.pan_id,
        args.radio_channel,
    );

    network_manager
        .configure(device_config)
        .await
        .expect("Failed to initialize network manager");
    network_manager
        .start(args.reinitialize)
        .await
        .expect("Failed to start network manager");

    info!(
        "Network initialized. Permitting joining for {} seconds...",
        args.join_secs
    );

    network_manager
        .allow_joins(args.join_secs.into())
        .await
        .expect("Failed to allow joins");

    info!("Waiting for network to open...");
    network_manager.await_network_open().await;
    info!("Network is open.");

    info!("Waiting for network to close...");
    network_manager.await_network_closed().await;
    info!("Joining period has ended.");

    info!("Sending active endpoint request...");
    active_endpoint_request(&mut network_manager, 0x01).await;

    info!("Switching off all connected devices...");
    switch_off(&mut network_manager, 0x02).await;

    info!("Waiting 10 seconds...");
    sleep(Duration::from_secs(10)).await;

    info!("Waiting 10 seconds...");
    sleep(Duration::from_secs(10)).await;

    for neighbor in network_manager
        .get_neighbors()
        .await
        .expect("Failed to get neighbors")
    {
        info!("Neighbor: {neighbor:?}");
    }

    info!("Sending active endpoint request...");
    active_endpoint_request(&mut network_manager, 0x03).await;

    info!("Switching off all connected devices...");
    switch_off(&mut network_manager, 0x04).await;

    info!("Enter node ID...");
    for line in stdin().lines().map_while(Result::ok) {
        let Ok(node_id) = line.trim().parse::<u16>() else {
            error!("Invalid node ID. Please enter a valid u16 value.");
            continue;
        };

        let aps_options = aps::Options::RETRY
            | aps::Options::ENABLE_ROUTE_DISCOVERY
            | aps::Options::ENABLE_ADDRESS_DISCOVERY;
        let aps_frame = aps::Frame::new(
            HOME_AUTOMATION,
            <Off as Cluster>::ID,
            0x01,
            0x01,
            aps_options,
            0x00,
            0x00,
        );
        let zcl_frame = zcl::Frame::new(
            zcl::Type::ClusterSpecific,
            zcl::Direction::ClientToServer,
            true,
            None,
            0x00,
            Off,
        );

        network_manager
            .send_unicast(
                Destination::Direct(node_id),
                aps_frame,
                &zcl_frame.to_le_stream().collect::<Vec<_>>(),
            )
            .await
            .expect("Failed to send unicast data");
    }
}

async fn move_to_color<T>(network_manager: &mut NetworkManager<T>, sequence: u8)
where
    T: Messaging + Networking,
{
    let move_to_color = MoveToColor::new(0x529E, 0x543B, 0, 0x00, 0x00);
    let aps_options = aps::Options::RETRY
        | aps::Options::ENABLE_ROUTE_DISCOVERY
        | aps::Options::ENABLE_ADDRESS_DISCOVERY;
    let aps_frame = aps::Frame::new(
        HOME_AUTOMATION,
        <MoveToColor as Cluster>::ID,
        0x01,
        0x01,
        aps_options,
        0x00,
        sequence,
    );
    let zcl_frame = zcl::Frame::new(
        zcl::Type::ClusterSpecific,
        zcl::Direction::ClientToServer,
        false,
        None,
        sequence,
        move_to_color,
    );
}

async fn trigger_effect<T>(network_manager: &mut NetworkManager<T>, sequence: u8)
where
    T: Messaging + Networking,
{
    let trigger_effect = TriggerEffect::new(EffectIdentifier::Blink, EffectVariant::Default);
    let aps_options = aps::Options::RETRY
        | aps::Options::ENABLE_ROUTE_DISCOVERY
        | aps::Options::ENABLE_ADDRESS_DISCOVERY;
    let aps_frame = aps::Frame::new(
        HOME_AUTOMATION,
        <TriggerEffect as Cluster>::ID,
        0x01,
        0x01,
        aps_options,
        0x00,
        sequence,
    );
    let zcl_frame = zcl::Frame::new(
        zcl::Type::ClusterSpecific,
        zcl::Direction::ClientToServer,
        false,
        None,
        sequence,
        trigger_effect,
    );

    send_to_all(network_manager, aps_frame, zcl_frame).await;
}

async fn switch_off<T>(network_manager: &mut NetworkManager<T>, sequence: u8)
where
    T: Messaging + Networking,
{
    let aps_options = aps::Options::RETRY
        | aps::Options::ENABLE_ROUTE_DISCOVERY
        | aps::Options::ENABLE_ADDRESS_DISCOVERY;
    let aps_frame = aps::Frame::new(
        HOME_AUTOMATION,
        <Off as Cluster>::ID,
        0x01,
        0x01,
        aps_options,
        0x00,
        sequence,
    );
    let zcl_frame = zcl::Frame::new(
        zcl::Type::ClusterSpecific,
        zcl::Direction::ClientToServer,
        true,
        None,
        sequence,
        Off,
    );

    send_to_all(network_manager, aps_frame, zcl_frame).await;
}

async fn active_endpoint_request<T>(network_manager: &mut NetworkManager<T>, sequence: u8)
where
    T: Messaging + Networking,
{
    let aps_options = aps::Options::RETRY
        | aps::Options::ENABLE_ROUTE_DISCOVERY
        | aps::Options::ENABLE_ADDRESS_DISCOVERY;
    let aps_frame = aps::Frame::new(0x0000, 0x0005, 0x01, 0x01, aps_options, 0x00, sequence);

    let mut payload = Vec::new();

    for (_, short_address) in network_manager
        .get_neighbors()
        .await
        .expect("Failed to get neighbors")
    {
        payload.clear();
        payload.push(0x00); // ZDP SEQ
        payload.extend(short_address.to_le_stream()); // NWK Address
        info!("Sending unicast to device with short ID {short_address}");
        debug!("APS Frame: {aps_frame:#06X?}");
        debug!("Payload: {:#04X}", HexSlice::new(&payload));
        network_manager
            .send_unicast(
                Destination::Direct(short_address),
                aps_frame.clone(),
                &payload,
            )
            .await
            .expect("Failed to send unicast data");
    }
}

async fn send_to_all<N, F>(
    network_manager: &mut NetworkManager<N>,
    aps_frame: aps::Frame,
    zcl_frame: zcl::Frame<F>,
) where
    N: Messaging + Networking,
    F: Clone + zcl::Command + ToLeStream,
{
    debug!("Sending APS frame: {aps_frame:#X?}");
    let payload = zcl_frame.to_le_stream().collect::<Vec<_>>();
    debug!("Sending payload: {payload:#04X?}");

    let mut targets: Vec<_> = network_manager
        .get_neighbors()
        .await
        .expect("Failed to get neighbors")
        .into_values()
        .collect();
    targets.extend(
        network_manager
            .get_children()
            .await
            .expect("Failed to get neighbors")
            .into_values(),
    );

    for short_address in targets {
        info!("Sending unicast to: {short_address}");
        network_manager
            .send_unicast(
                Destination::Direct(short_address),
                aps_frame.clone(),
                &payload,
            )
            .await
            .expect("Failed to send unicast data");
    }
}
