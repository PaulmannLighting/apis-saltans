use std::sync::Arc;

use ezsp::ember::aps;
use ezsp::ember::message::Destination;
use ezsp::uart::Uart;
use ezsp::zigbee::NetworkManager;
use le_stream::ToLeStream;
use rocket::serde::json::Json;
use rocket::{State, get, put};
use serialport::TTYPort;
use tokio::sync::Mutex;
use zcl::Cluster;
use zcl::general::on_off::{Off, On};
use zcl::lighting::color_control::MoveToColor;

use self::device::Device;
use self::ezsp_json_response::EzspJsonResponse;
use crate::HOME_AUTOMATION;
use crate::web_api::color_move::ColorMove;

mod color_move;
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

#[get("/switch-off/<short_address>")]
pub async fn switch_off(zigbee: &State<Zigbee>, short_address: u16) -> EzspJsonResponse<u8> {
    zigbee
        .lock()
        .await
        .send_unicast(
            Destination::Direct(short_address),
            aps::Frame::new(
                HOME_AUTOMATION,
                <Off as Cluster>::ID,
                0x01,
                0x01,
                aps::Options::RETRY
                    | aps::Options::ENABLE_ROUTE_DISCOVERY
                    | aps::Options::ENABLE_ADDRESS_DISCOVERY,
                0x00,
                0x00,
            ),
            zcl::Frame::new(
                zcl::Type::ClusterSpecific,
                zcl::Direction::ClientToServer,
                true,
                None,
                0x00,
                Off,
            )
            .to_le_stream(),
        )
        .await
        .into()
}

#[get("/switch-on/<short_address>")]
pub async fn switch_on(zigbee: &State<Zigbee>, short_address: u16) -> EzspJsonResponse<u8> {
    zigbee
        .lock()
        .await
        .send_unicast(
            Destination::Direct(short_address),
            aps::Frame::new(
                HOME_AUTOMATION,
                <On as Cluster>::ID,
                0x01,
                0x01,
                aps::Options::RETRY
                    | aps::Options::ENABLE_ROUTE_DISCOVERY
                    | aps::Options::ENABLE_ADDRESS_DISCOVERY,
                0x00,
                0x00,
            ),
            zcl::Frame::new(
                zcl::Type::ClusterSpecific,
                zcl::Direction::ClientToServer,
                true,
                None,
                0x00,
                On,
            )
            .to_le_stream(),
        )
        .await
        .into()
}

#[put("/set-color/<short_address>", data = "<color_move>")]
pub async fn set_color(
    zigbee: &State<Zigbee>,
    short_address: u16,
    color_move: Json<ColorMove>,
) -> EzspJsonResponse<u8> {
    zigbee
        .lock()
        .await
        .send_unicast(
            Destination::Direct(short_address),
            aps::Frame::new(
                HOME_AUTOMATION,
                <MoveToColor as Cluster>::ID,
                0x01,
                0x01,
                aps::Options::RETRY
                    | aps::Options::ENABLE_ROUTE_DISCOVERY
                    | aps::Options::ENABLE_ADDRESS_DISCOVERY,
                0x00,
                0x00,
            ),
            zcl::Frame::new(
                zcl::Type::ClusterSpecific,
                zcl::Direction::ClientToServer,
                false,
                None,
                0x00,
                MoveToColor::from(color_move.into_inner()),
            )
            .to_le_stream(),
        )
        .await
        .into()
}
