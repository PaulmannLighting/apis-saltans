//! Coordinator-owned OTA Upgrade server.

use bytes::Bytes;
use le_stream::ToLeStream;
use log::warn;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use zb_aps::TxOptions;
use zb_aps::apsde::{
    DataRequest, IndividualEndpoint, NetworkAddress, NetworkDestination, RequestDestination,
};
use zb_core::{Cluster, Direction, Profile};
use zb_zcl::{Command, Directed, Scope, Scoped, UnsequencedFrame};

pub use self::image::{
    BaseHeaderBytes, FieldControl, Header, HeaderString, Image, ParseImage, ParseImageError,
};
pub use self::message::{Message, UpdateError, UpdateResult};
pub use self::server::Server;
pub use self::timeouts::UpdateTimeouts;
pub use self::update::CancellableOtaUpdate;
pub(crate) use self::update::Update;
use crate::aps::TransmissionResponse;
use crate::{Error, zcl};

mod image;
mod message;
mod page_transfer;
mod server;
mod state;
mod timeouts;
mod transfer;
mod update;

const CURRENT_TIME_IMMEDIATE: u32 = 0;
const UPGRADE_TIME_IMMEDIATE: u32 = 0;
const OTA_PROFILE: Profile = Profile::ZigbeeHomeAutomation;
#[cfg(test)]
const TEST_IEEE_ADDRESS: zb_core::IeeeAddress =
    zb_core::IeeeAddress::new(0x00, 0x12, 0x4b, 0x00, 0x01, 0xaa, 0xbb, 0xcc);

type Request = DataRequest<UnsequencedFrame<Bytes>>;

const fn network_destination(
    short_id: zb_core::short_id::Device,
    endpoint: IndividualEndpoint,
) -> NetworkDestination {
    NetworkDestination::new(
        NetworkAddress::new(short_id.as_u16())
            .expect("device short addresses are valid APSDE network addresses"),
        endpoint,
    )
}

fn request<T>(
    destination: RequestDestination,
    source_endpoint: IndividualEndpoint,
    profile: Profile,
    cluster_id: u16,
    command: T,
) -> Request
where
    T: Command + Directed + Scoped + ToLeStream,
{
    request_from_unsequenced_frame(
        destination,
        source_endpoint,
        profile,
        cluster_id,
        UnsequencedFrame::from_command(command),
    )
}

const fn request_from_unsequenced_frame(
    destination: RequestDestination,
    source_endpoint: IndividualEndpoint,
    profile: Profile,
    cluster_id: u16,
    frame: UnsequencedFrame<Bytes>,
) -> Request {
    DataRequest::new(
        destination,
        profile.as_u16(),
        cluster_id,
        source_endpoint,
        frame,
    )
    .with_tx_options(TxOptions::ACKNOWLEDGED_TRANSMISSION)
}

pub(crate) fn subscription() -> (zcl::Subscription, zcl::SubscriptionReceiver) {
    zcl::Subscription::channel(zcl::SubscriptionFilter::new(
        Cluster::OtaUpgrade,
        Scope::ClusterSpecific,
        Direction::ClientToServer,
    ))
}

async fn reply_zcl(
    zcl: &Sender<zcl::Message>,
    sequence_number: u8,
    request: Request,
) -> Option<()> {
    let (response, result) = oneshot::channel();
    if let Err(error) = zcl
        .send(zcl::Message::Reply {
            sequence_number,
            request,
            response,
        })
        .await
    {
        warn!("Failed to queue OTA reply: {error}");
        return None;
    }
    receive_transmission_result(result).await
}

async fn send_zcl(zcl: &Sender<zcl::Message>, request: Request) -> Option<()> {
    let (response, result) = oneshot::channel();
    if let Err(error) = zcl.send(zcl::Message::Transmit { request, response }).await {
        warn!("Failed to queue OTA command: {error}");
        return None;
    }
    receive_transmission_result(result).await
}

async fn receive_transmission_result(
    response: oneshot::Receiver<Result<TransmissionResponse, Error>>,
) -> Option<()> {
    let transmission = match response.await {
        Ok(Ok(transmission)) => transmission,
        Ok(Err(error)) => {
            warn!("Failed to queue OTA transmission: {error}");
            return None;
        }
        Err(error) => {
            warn!("Failed to receive OTA transmission result: {error}");
            return None;
        }
    };

    match transmission.await {
        Ok(()) => Some(()),
        Err(error) => {
            warn!("OTA transmission failed: {error}");
            None
        }
    }
}

#[cfg(test)]
mod tests;
