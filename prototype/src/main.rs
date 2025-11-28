//! A prototype for testing the EZSP UART implementation.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::time::Duration;

use ashv2::{BaudRate, open};
use clap::Parser;
use ezsp::ember::message::Outgoing;
use ezsp::ember::security::initial;
use ezsp::ember::{Status, aps, concentrator, join, network};
use ezsp::ezsp::{config, decision, policy};
use ezsp::parameters::networking::handler::Handler::{ChildJoin, StackStatus};
use ezsp::uart::Uart;
use ezsp::{Callback, Configuration, Error, Ezsp, Messaging, Networking, Security, Utilities};
use le_stream::ToLeStream;
use log::{info, warn};
use macaddr::MacAddr8;
use serialport::FlowControl;
use tokio::sync::mpsc::Receiver;
use tokio::time::sleep;
use zdp::MgmtPermitJoiningReq;

const ENDPOINT_ID: u8 = 1;
const RADIO_TX_POWER: i8 = 8;
const PAN_ID: u16 = 24171;
const EXTENDED_PAN_ID: MacAddr8 = MacAddr8::new(0x8D, 0x9F, 0x3D, 0xFE, 0x00, 0xBF, 0x0D, 0xB5);
const NETWORK_KEY: [u8; 16] = [
    0x29, 0xB0, 0x0D, 0xE6, 0x31, 0xAB, 0x7A, 0xD0, 0xC6, 0x83, 0xC8, 0x7A, 0xBF, 0x70, 0xD6, 0x08,
];
const RADIO_CHANNEL: u8 = 11;
const LINK_KEY: &[u8] = include_bytes!("../../assets/link.key");
const HOME_AUTOMATION: u16 = 0x0104;
const HOME_GATEWAY: u16 = 0x0050;
const INPUT_CLUSTERS: &[u16] = &[0x0000, 0x0006, 0x0008, 0x0300, 0x0403, 0x0201];
const OUTPUT_CLUSTERS: &[u16] = &[0x0000, 0x0006, 0x0008, 0x0300, 0x0403];

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

    // Spawn a task to handle incoming callbacks.
    tokio::spawn(handle_callbacks(
        callbacks_receiver,
        network_up.clone(),
        network_open.clone(),
    ));

    let mut uart = Uart::new(serial_port, callbacks_sender, 8, 1024);
    uart.init().await.expect("Failed to initialize UART");

    info!("Adding endpoint");
    add_endpoint(&mut uart)
        .await
        .expect("Failed to add endpoint");

    if args.reinitialize {
        info!("Reinitializing network");
        reinitialize_network(&mut uart, &args, network_up.clone())
            .await
            .expect("Failed to reinitialize network");
    } else {
        info!("Initializing network");
        initialize(&mut uart)
            .await
            .expect("Failed to initialize network");
        uart.network_init(BTreeSet::default())
            .await
            .expect("Network init failed");
    }

    info!("Waiting for network to come up...");
    while !network_up.load(SeqCst) {
        sleep(Duration::from_secs(1)).await;
    }
    info!("Network is up");

    let (node_type, parameters) = uart
        .get_network_parameters()
        .await
        .expect("Failed to get network parameters");
    info!("Node type: {node_type:?}");
    log_parameters(&parameters);

    let security_state = uart
        .get_current_security_state()
        .await
        .expect("Failed to get current security state");
    info!("Security state bitmask: {:#06X}", security_state.bitmask());

    info!("Setting radio TX power to {RADIO_TX_POWER}");
    uart.set_radio_power(RADIO_TX_POWER)
        .await
        .expect("Failed to set radio power");

    let node_id = uart.get_node_id().await.expect("Failed to get node ID");
    info!("Node ID: {node_id:#06X}");

    info!("Sending many-to-one route request");
    uart.send_many_to_one_route_request(concentrator::Type::HighRam, 8)
        .await
        .expect("Failed to send many-to-one route request");

    let packet_buffer_count = uart
        .get_configuration_value(config::Id::PacketBufferCount)
        .await
        .expect("Failed to get packet buffer count");
    info!("Packet buffer count: {packet_buffer_count}");

    info!("Permitting joining for {} seconds", args.join_secs);
    uart.permit_joining(args.join_secs.into())
        .await
        .expect("Failed to permit joining");

    info!("Waiting for network to open...");
    while !network_open.load(SeqCst) {
        sleep(Duration::from_secs(1)).await;
    }
    info!("Network is opened");

    let seq = send_broadcast(&mut uart, args.join_secs)
        .await
        .expect("Failed to send broadcast");
    info!("Sent broadcast with sequence number: {seq}");

    let seq = send_unicast(&mut uart, args.join_secs)
        .await
        .expect("Failed to send unicast");
    info!("Sent unicast with sequence number: {seq}");

    tokio::spawn(async move {
        loop {
            match uart.network_state().await {
                Ok(state) => info!("Network state: {state:?}"),
                Err(error) => warn!("Failed to get network state: {error}"),
            }

            for index in 0..=u8::MAX {
                if let Ok(child) = uart.get_child_data(index).await {
                    info!("Child at {index}: {child:?}");
                } else {
                    break;
                }
            }

            for index in 0..=u8::MAX {
                if let Ok(neighbor) = uart.get_neighbor(index).await {
                    info!("Neighbor at {index}: {neighbor:?}");
                } else {
                    break;
                }
            }

            sleep(Duration::from_secs(1)).await;
        }
    });

    info!("Waiting for network to close...");
    while network_open.load(SeqCst) {
        sleep(Duration::from_secs(1)).await;
    }
    info!("Network is closed");
}

async fn reinitialize_network<T>(
    uart: &mut T,
    args: &Args,
    network_up: Arc<AtomicBool>,
) -> Result<(), Error>
where
    T: Configuration + Security + Networking + Utilities,
{
    if matches!(uart.leave_network().await, Ok(())) {
        info!("Leaving existing network");

        while network_up.load(SeqCst) {
            sleep(Duration::from_secs(1)).await;
        }

        info!("Left existing network");
    }

    initialize(uart).await?;
    uart.form_network(network::Parameters::new(
        args.extended_pan_id,
        args.pan_id,
        8,
        args.radio_channel,
        join::Method::MacAssociation,
        0,
        0,
        0,
    ))
    .await
}

async fn add_endpoint<T>(uart: &mut T) -> Result<(), Error>
where
    T: Configuration,
{
    info!("Adding endpoint");
    uart.add_endpoint(
        ENDPOINT_ID,
        HOME_AUTOMATION,
        HOME_GATEWAY,
        0,
        INPUT_CLUSTERS.iter().copied().collect(),
        OUTPUT_CLUSTERS.iter().copied().collect(),
    )
    .await
}

async fn initialize<T>(uart: &mut T) -> Result<(), Error>
where
    T: Configuration + Security + Networking + Utilities,
{
    info!("Initializing EZSP NCP");

    let config = concentrator::Parameters::new(
        concentrator::Type::HighRam,
        Duration::from_secs(60),
        Duration::from_secs(3600),
        8,
        8,
        0,
    )
    .expect("Concentrator parameters should be valid.");
    uart.set_concentrator(Some(config)).await?;

    let mut configuration = BTreeMap::new();
    configuration.insert(config::Id::SourceRouteTableSize, 100);
    configuration.insert(config::Id::ApsUnicastMessageCount, 16);
    configuration.insert(config::Id::NeighborTableSize, 24);
    configuration.insert(config::Id::MaxHops, 30);
    uart.set_stack_configuration(configuration).await?;

    let mut policy = BTreeMap::new();
    policy.insert(policy::Id::TrustCenter, decision::Id::AllowJoins);
    uart.set_stack_policy(policy).await?;

    let ieee_address = uart.get_eui64().await?;
    info!("IEEE address: {ieee_address}");

    uart.set_radio_power(8).await?;

    uart.set_initial_security_state(initial::State::new(
        [
            initial::Bitmask::HavePreconfiguredKey,
            initial::Bitmask::RequireEncryptedKey,
        ]
        .into(),
        LINK_KEY.try_into().expect("Link key should be valid."),
        NETWORK_KEY,
        0,
        MacAddr8::default(),
    ))
    .await?;

    Ok(())
}

async fn send_broadcast<T>(uart: &mut T, join_secs: u8) -> Result<u8, Error>
where
    T: Messaging,
{
    let zdp_frame = zdp::Frame::new(0, MgmtPermitJoiningReq::new(join_secs, true));
    let aps_options = aps::Options::RETRY
        | aps::Options::ENABLE_ADDRESS_DISCOVERY
        | aps::Options::ENABLE_ROUTE_DISCOVERY;
    let aps_frame = aps::Frame::new(0, zdp_frame.cluster_id(), 0, 0, aps_options, 0, 0);
    uart.send_broadcast(0xFFFC, aps_frame, 31, 5, zdp_frame.to_le_stream().collect())
        .await
}

async fn send_unicast<T>(uart: &mut T, join_secs: u8) -> Result<u8, Error>
where
    T: Messaging,
{
    let zdp_frame = zdp::Frame::new(0, MgmtPermitJoiningReq::new(join_secs, true));
    let aps_options = aps::Options::RETRY
        | aps::Options::ENABLE_ADDRESS_DISCOVERY
        | aps::Options::ENABLE_ROUTE_DISCOVERY;
    let aps_frame = aps::Frame::new(0, zdp_frame.cluster_id(), 0, 1, aps_options, 0, 0);
    uart.send_unicast(
        Outgoing::Direct,
        PAN_ID,
        aps_frame,
        5,
        zdp_frame.to_le_stream().collect(),
    )
    .await
}

async fn handle_callbacks(
    mut callbacks: Receiver<Callback>,
    network_up: Arc<AtomicBool>,
    network_open: Arc<AtomicBool>,
) {
    while let Some(callback) = callbacks.recv().await {
        info!("Received callback: {callback:?}");

        if let Callback::Networking(StackStatus(status)) = &callback {
            match status.result() {
                Ok(Status::NetworkUp) => {
                    network_up.store(true, SeqCst);
                }
                Ok(Status::NetworkDown) => {
                    network_up.store(false, SeqCst);
                }
                Ok(Status::NetworkOpened) => {
                    network_open.store(true, SeqCst);
                }
                Ok(Status::NetworkClosed) => {
                    network_open.store(false, SeqCst);
                }
                _ => (),
            }
        }

        if let Callback::Networking(ChildJoin(status)) = &callback {
            info!("Child joined with ID: {}", status.child_id());
        }
    }

    warn!("Callback handler exiting");
}

fn log_parameters(parameters: &network::Parameters) {
    info!("PAN ID: {:#X}", parameters.pan_id());
    info!("Extended PAN ID: {:#X?}", parameters.extended_pan_id());
    info!("Radio TX power: {:#X}", parameters.radio_tx_power());
    info!("Radio channel: {:#X}", parameters.radio_channel());
    info!("Join method: {:#X?}", parameters.join_method());
    info!("Nwk manager ID: {:#X}", parameters.nwk_manager_id());
    info!("Nwk update ID: {:#X}", parameters.nwk_update_id());
    info!("Channels: {:#X}", parameters.channels());
}

trait SetStackConfiguration {
    fn set_stack_configuration(
        &mut self,
        configuration: BTreeMap<config::Id, u16>,
    ) -> impl Future<Output = Result<(), Error>>;
}

impl<T> SetStackConfiguration for T
where
    T: Configuration,
{
    async fn set_stack_configuration(
        &mut self,
        configuration: BTreeMap<config::Id, u16>,
    ) -> Result<(), Error> {
        for (key, value) in configuration {
            info!("Setting configuration {key:?} to {value}");
            self.set_configuration_value(key, value).await?;
        }

        Ok(())
    }
}

trait SetStackPolicy {
    fn set_stack_policy(
        &mut self,
        policy: BTreeMap<policy::Id, decision::Id>,
    ) -> impl Future<Output = Result<(), Error>>;
}

impl<T> SetStackPolicy for T
where
    T: Configuration,
{
    async fn set_stack_policy(
        &mut self,
        policy: BTreeMap<policy::Id, decision::Id>,
    ) -> Result<(), Error> {
        for (key, value) in policy {
            info!("Setting policy {key:?} to {value:?}");
            self.set_policy(key, value).await?;
        }

        Ok(())
    }
}
