use std::sync::Arc;
use std::time::Duration;

use ezsp::uart::Uart;
use ezsp::zigbee::NetworkManager;
use rocket::serde::json::Json;
use rocket::{State, get, put};
use serialport::TTYPort;
use tokio::sync::Mutex;
use zcl::general::on_off::{Off, On};
use zcl::lighting::color_control::MoveToColor;
use zigbee_nwk::Nlme;
use zigbee_nwk::aps::{Command, Destination};

use self::color_move::ColorMove;
use self::device::Device;
use self::json_result::JsonResult;

mod color_move;
mod device;
mod json_result;

type Zigbee = Arc<Mutex<NetworkManager<Uart<TTYPort>>>>;

#[get("/allow-join")]
pub async fn allow_join(zigbee: &State<Zigbee>) -> JsonResult<(), zigbee_nwk::Error<ezsp::Error>> {
    zigbee
        .lock()
        .await
        .allow_joins(Duration::from_secs(60))
        .await
        .into()
}

#[get("/neighbors")]
pub async fn get_neighbors(
    zigbee: &State<Zigbee>,
) -> JsonResult<Vec<Device>, zigbee_nwk::Error<ezsp::Error>> {
    let neighbors = match zigbee.lock().await.get_neighbors().await {
        Ok(neighbors) => neighbors,
        Err(err) => return Err(err).into(),
    };

    let neighbors: Vec<_> = neighbors
        .into_iter()
        .map(|(mac_address, short_id)| Device::new(mac_address, short_id))
        .collect();
    Ok(neighbors).into()
}

#[get("/switch-off/<short_address>")]
pub async fn switch_off(
    zigbee: &State<Zigbee>,
    short_address: u16,
) -> JsonResult<(), zigbee_nwk::Error<ezsp::Error>> {
    zigbee
        .lock()
        .await
        .unicast_command(
            short_address,
            Command::new(Destination::Unicast(0x01), 0x00, Off),
        )
        .await
        .into()
}

#[get("/switch-on/<short_address>")]
pub async fn switch_on(
    zigbee: &State<Zigbee>,
    short_address: u16,
) -> JsonResult<(), zigbee_nwk::Error<ezsp::Error>> {
    zigbee
        .lock()
        .await
        .unicast_command(
            short_address,
            Command::new(Destination::Unicast(0x01), 0x00, On),
        )
        .await
        .into()
}

#[put("/set-color/<short_address>", data = "<color_move>")]
pub async fn set_color(
    zigbee: &State<Zigbee>,
    short_address: u16,
    color_move: Json<ColorMove>,
) -> JsonResult<(), zigbee_nwk::Error<ezsp::Error>> {
    zigbee
        .lock()
        .await
        .unicast_command(
            short_address,
            Command::new(
                Destination::Unicast(0x01),
                0x00,
                MoveToColor::from(color_move.into_inner()),
            ),
        )
        .await
        .into()
}
