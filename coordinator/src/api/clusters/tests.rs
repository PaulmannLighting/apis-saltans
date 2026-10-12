use std::pin::pin;
use std::task::{Context, Poll, Waker};

use tokio::sync::mpsc::{Receiver, channel};
use tokio::sync::oneshot;
use zb_aps::apsde::{IndividualEndpoint, NetworkAddress, NetworkDestination};
use zb_core::endpoint::Application;
use zb_core::{Direction, Endpoint};
use zb_zcl::on_off::SendReport;
use zb_zcl::{Cluster, Discrete};

use crate::aps::TransmissionResponse;
use crate::correlation::Cancellation;
use crate::response::ApsProtocolResponse;
use crate::zcl::{Message, ZclKey};
use crate::{Attributes, Error, Groups};

const CAPACITY: usize = 1;
const ADDRESS: u16 = 0x1234;
const CLUSTER: u16 = 6;
const PROFILE: u16 = 0x0104;
const SEQUENCE: u8 = 0;
const INTERVAL: u16 = 10;

fn endpoint() -> IndividualEndpoint {
    IndividualEndpoint::new(Endpoint::Application(Application::MIN)).expect("individual endpoint")
}

fn destination() -> NetworkDestination {
    NetworkDestination::new(
        NetworkAddress::new(ADDRESS).expect("network address"),
        endpoint(),
    )
}

#[test]
fn group_listing_awaits_transmission_and_protocol_response() {
    let (sender, messages) = channel(CAPACITY);
    check_completion(sender.list(destination(), endpoint()), messages);
}

#[test]
fn reporting_configuration_awaits_transmission_and_protocol_response() {
    let (sender, messages) = channel(CAPACITY);
    check_completion(
        sender.configure_reporting(
            destination(),
            endpoint(),
            [SendReport::OnOff(Discrete::new(INTERVAL, INTERVAL))],
        ),
        messages,
    );
}

fn check_completion<F, T>(future: F, mut messages: Receiver<Message>)
where
    F: Future<Output = Result<T, Error>>,
{
    let mut future = pin!(future);
    let mut context = Context::from_waker(Waker::noop());
    assert!(future.as_mut().poll(&mut context).is_pending());
    let Message::Communicate { response, .. } = messages.try_recv().expect("request queued") else {
        panic!("expected communication");
    };
    let (accepted, transmission) = oneshot::channel();
    let (received, protocol) = oneshot::channel::<Result<Cluster, Error>>();
    let (aps, _messages) = channel(CAPACITY);
    let key = ZclKey {
        short_id: ADDRESS,
        remote_endpoint: endpoint().get(),
        local_endpoint: endpoint().get(),
        cluster_id: CLUSTER,
        profile_id: PROFILE,
        manufacturer_code: None,
        direction: Direction::ServerToClient,
        sequence: SEQUENCE,
    };
    assert!(
        response
            .send(Ok(ApsProtocolResponse::new(
                TransmissionResponse::test_new(transmission, SEQUENCE, aps.downgrade()),
                protocol,
                Cancellation::test_new(key, |_| {}),
            )))
            .is_ok()
    );
    assert!(future.as_mut().poll(&mut context).is_pending());
    accepted.send(Ok(())).expect("transmission awaited");
    assert!(future.as_mut().poll(&mut context).is_pending());
    received
        .send(Err(Error::ProtocolResponseTimeout))
        .expect("response awaited");
    assert!(matches!(
        future.as_mut().poll(&mut context),
        Poll::Ready(Err(Error::ProtocolResponseTimeout))
    ));
}
