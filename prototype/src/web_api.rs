use std::sync::Arc;

use ezsp::ember::aps;
use ezsp::ember::message::Destination;
use ezsp::ember::network::Duration;
use ezsp::uart::Uart;
use ezsp::zigbee::NetworkManager;
use ezsp::{Error, Messaging, Networking};
use le_stream::ToLeStream;
use rocket::http::ext::IntoOwned;
use rocket::{State, get};
use serialport::TTYPort;
use tokio::sync::Mutex;
use zcl::Cluster;
use zcl::lighting::color_control::MoveToColor;

use self::device::Device;
use self::ezsp_json_response::EzspJsonResponse;
use crate::HOME_AUTOMATION;

mod device;
mod ezsp_json_response;

type Zigbee = Arc<Mutex<NetworkManager<Uart<TTYPort>>>>;

#[get("/allow-join")]
pub async fn allow_join(zigbee: &State<Zigbee>) -> EzspJsonResponse<()> {
    zigbee.lock().await.allow_joins(60u8.into()).await.into()
}

#[get("/neighbors")]
pub async fn get_neighbors(zigbee: &State<Zigbee>) -> EzspJsonResponse<Vec<Device>> {
    let neighbors = match zigbee.lock().await.get_neighbors().await {
        Ok(neighbors) => neighbors,
        Err(err) => return Err(err).into(),
    };

    let neighbors: Vec<_> = neighbors
        .into_iter()
        .map(|(mac_address, short_id)| Device::new(mac_address, Some(short_id)))
        .collect();
    Ok(neighbors).into()
}

#[get("/set-color/<short_address>")]
pub async fn set_color(zigbee: &State<Zigbee>, short_address: u16) -> EzspJsonResponse<u8> {
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
        0x00,
    );
    let zcl_frame = zcl::Frame::new(
        zcl::Type::ClusterSpecific,
        zcl::Direction::ClientToServer,
        false,
        None,
        0x00,
        move_to_color,
    );

    zigbee
        .lock()
        .await
        .send_unicast(
            Destination::Direct(short_address),
            aps_frame.clone(),
            zcl_frame.clone().to_le_stream(),
        )
        .await
        .into()
}
