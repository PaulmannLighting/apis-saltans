//! A prototype for testing the EZSP UART implementation.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::time::Duration;

use ashv2::{BaudRate, open};
use clap::Parser;
use ezsp::ember::security::initial;
use ezsp::ember::{Status, concentrator, join, network};
use ezsp::ezsp::{config, decision, policy};
use ezsp::parameters::networking::handler::Handler::StackStatus;
use ezsp::uart::Uart;
use ezsp::{Callback, Configuration, Error, Ezsp, Messaging, Networking, Security, Utilities};
use log::{error, info};
use macaddr::MacAddr8;
use serialport::FlowControl;
use tokio::sync::mpsc::Receiver;
use tokio::time::sleep;

const PAN_ID: u16 = 0x5EAB;
const EXTENDED_PAN_ID: MacAddr8 = MacAddr8::new(0x8D, 0x9F, 0x3D, 0xFE, 0x00, 0xBF, 0x0D, 0xB5);
const RADIO_CHANNEL: u8 = 12;
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
    let network_closed = Arc::new(AtomicBool::new(false));

    let serial_port = open(args.tty.clone(), BaudRate::RstCts, FlowControl::Software)
        .expect("Failed to open serial port");
    let (callbacks_sender, callbacks_receiver) = tokio::sync::mpsc::channel(1024);

    // Spawn a task to handle incoming callbacks.
    tokio::spawn(handle_callbacks(
        callbacks_receiver,
        network_up.clone(),
        network_closed.clone(),
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
        uart.network_init(BTreeSet::default())
            .await
            .expect("Failed to initialize network");
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

    info!("Sending many-to-one route request");
    uart.send_many_to_one_route_request(concentrator::Type::HighRam, 8)
        .await
        .expect("Failed to send many-to-one route request");

    info!("Permitting joining for {} seconds", args.join_secs);
    uart.permit_joining(args.join_secs.into())
        .await
        .expect("Failed to permit joining");

    info!("Waiting for network to close...");
    while !network_closed.load(SeqCst) {
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
        0,
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

    info!("Getting current network parameters");
    match uart.get_network_parameters().await {
        Ok((node_type, parameters)) => {
            info!("Node type: {node_type:?}");
            log_parameters(&parameters);
        }
        Err(error) => {
            error!("Failed to get network parameters: {error}");
        }
    }

    let ieee_address = uart.get_eui64().await?;
    info!("IEEE address: {ieee_address}");

    uart.set_radio_power(8).await?;

    let link_key = LINK_KEY
        .try_into()
        .expect("Link key should be valid. This is a bug.");
    info!("Link key: {link_key:02X?}");

    // Randomly generated network key for testing.
    let network_key = [
        0xCB, 0x61, 0x6A, 0x55, 0xA2, 0xA6, 0x9D, 0x7D, 0x7F, 0x71, 0x4F, 0xAD, 0x88, 0xA5, 0xD4,
        0x9E,
    ];
    info!("Network key: {network_key:02X?}");
    uart.set_initial_security_state(initial::State::new(
        [
            initial::Bitmask::HavePreconfiguredKey,
            initial::Bitmask::RequireEncryptedKey,
        ]
        .into(),
        link_key,
        network_key,
        0,
        MacAddr8::default(),
    ))
    .await?;

    Ok(())
}

async fn handle_callbacks(
    mut callbacks: Receiver<Callback>,
    network_up: Arc<AtomicBool>,
    network_closed: Arc<AtomicBool>,
) {
    while let Some(callback) = callbacks.recv().await {
        info!("Received callback: {callback:?}");

        if let Callback::Networking(StackStatus(status)) = &callback {
            match status.result() {
                Ok(Status::NetworkUp) => {
                    network_up.store(true, SeqCst);
                }
                Ok(Status::NetworkClosed) => {
                    network_closed.store(true, SeqCst);
                }
                Ok(Status::NetworkDown) => {
                    network_up.store(false, SeqCst);
                }
                _ => (),
            }
        }
    }
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
