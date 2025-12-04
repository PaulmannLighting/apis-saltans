use std::sync::Arc;
use std::time::Duration;

use ezsp::uart::Uart;
use ezsp::zigbee::NetworkManager;
use rand::SeedableRng;
use rand::prelude::IndexedRandom;
use rand::rngs::SmallRng;
use rocket::serde::json::Json;
use rocket::{State, get, put};
use serialport::TTYPort;
use tokio::sync::Mutex;
use zcl::general::on_off::{Off, On};
use zcl::lighting::color_control::MoveToColor;
use zigbee_nwk::Nlme;

use self::color_move::{ColorMove, Rgb};
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

#[get("/switch-off/<pan_id>")]
pub async fn switch_off(
    zigbee: &State<Zigbee>,
    pan_id: u16,
) -> JsonResult<(), zigbee_nwk::Error<ezsp::Error>> {
    zigbee
        .lock()
        .await
        .device(pan_id)
        .default_endpoint()
        .unicast_command(Off)
        .await
        .into()
}

#[get("/switch-on/<pan_id>")]
pub async fn switch_on(
    zigbee: &State<Zigbee>,
    pan_id: u16,
) -> JsonResult<(), zigbee_nwk::Error<ezsp::Error>> {
    zigbee
        .lock()
        .await
        .device(pan_id)
        .default_endpoint()
        .unicast_command(On)
        .await
        .into()
}

#[put("/set-color/<pan_id>", data = "<color_move>")]
pub async fn set_color(
    zigbee: &State<Zigbee>,
    pan_id: u16,
    color_move: Json<ColorMove>,
) -> JsonResult<(), zigbee_nwk::Error<ezsp::Error>> {
    zigbee
        .lock()
        .await
        .device(pan_id)
        .default_endpoint()
        .unicast_command(MoveToColor::from(color_move.into_inner()))
        .await
        .into()
}

#[put("/party")]
pub async fn party(zigbee: &State<Zigbee>) -> JsonResult<(), zigbee_nwk::Error<ezsp::Error>> {
    tokio::spawn(do_party(zigbee.inner().clone()));
    Ok(()).into()
}

async fn do_party(zigbee: Zigbee) -> Result<(), zigbee_nwk::Error<ezsp::Error>> {
    let colors = [
        Rgb::new(255, 0, 0),
        Rgb::new(0, 255, 0),
        Rgb::new(0, 0, 255),
    ];
    let neighbors = zigbee.lock().await.get_neighbors().await?;

    let delay_secs = 0;
    let mut rng = SmallRng::from_os_rng();

    for _ in 0..30 {
        for pan_id in neighbors.iter().filter_map(|(_, short_id)| *short_id) {
            zigbee
                .lock()
                .await
                .device(pan_id)
                .default_endpoint()
                .unicast_command(MoveToColor::from(ColorMove::new(
                    *colors.choose(&mut rng).expect("Colors are not empty"),
                    delay_secs * 10,
                )))
                .await?;
        }
    }

    Ok(())
}
