use std::pin::pin;
use std::task::{Context, Poll, Waker};

use tokio::sync::mpsc::{Receiver, channel};
use tokio::sync::oneshot;
use zb_aps::apsde::{IndividualEndpoint, NetworkAddress, NetworkDestination};
use zb_core::endpoint::Application;
use zb_core::types::Uint8;
use zb_core::{Direction, Endpoint, IeeeAddress};
use zb_zcl::ias::zone::{IasCieAddress, Writable};
use zb_zcl::on_off::SendReport;
use zb_zcl::{Cluster, Discrete, Frame};

use crate::aps::TransmissionResponse;
use crate::correlation::Cancellation;
use crate::response::ApsProtocolResponse;
use crate::zcl::{Message, ZclKey};
use crate::{Attributes, Error, Groups, WriteAttributeResult};

const CAPACITY: usize = 1;
const ADDRESS: u16 = 0x1234;
const SEQUENCE: u8 = 0;
const INTERVAL: u16 = 10;
const IAS_ZONE_CLUSTER: u16 = zb_core::Cluster::IasZone.as_u16();
const CIE_ADDRESS_ID: u16 = 0x0010;
const SENSITIVITY_ID: u16 = 0x0013;
const SUCCESSFUL_WRITE: [u8; 4] = [0x18, SEQUENCE, 0x04, 0x00];
const PARTIALLY_FAILED_WRITE: [u8; 6] = [0x18, SEQUENCE, 0x04, 0x86, 0x10, 0x00];

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
    let result = complete_request(
        sender.list(destination(), endpoint()),
        messages,
        Err(Error::ProtocolResponseTimeout),
    );
    assert!(matches!(
        result,
        Poll::Ready(Err(Error::ProtocolResponseTimeout))
    ));
}

#[test]
fn reporting_configuration_awaits_transmission_and_protocol_response() {
    let (sender, messages) = channel(CAPACITY);
    let result = complete_request(
        sender.configure_reporting(
            destination(),
            endpoint(),
            [SendReport::OnOff(Discrete::new(INTERVAL, INTERVAL))],
        ),
        messages,
        Err(Error::ProtocolResponseTimeout),
    );
    assert!(matches!(
        result,
        Poll::Ready(Err(Error::ProtocolResponseTimeout))
    ));
}

#[test]
fn writing_the_ias_cie_address_returns_its_id_on_success() {
    assert_eq!(
        complete_write([cie_address()], SUCCESSFUL_WRITE),
        [Ok(CIE_ADDRESS_ID)],
    );
}

#[test]
fn writing_multiple_attributes_returns_all_ids_in_request_order_on_success() {
    let attributes = [
        Writable::CurrentZoneSensitivityLevel(Uint8::MIN),
        cie_address(),
    ];
    assert_eq!(
        complete_write(attributes, SUCCESSFUL_WRITE),
        [Ok(SENSITIVITY_ID), Ok(CIE_ADDRESS_ID)],
    );
}

#[test]
fn writing_multiple_attributes_reports_partial_failure_in_request_order() {
    let attributes = [
        Writable::CurrentZoneSensitivityLevel(Uint8::MIN),
        cie_address(),
    ];
    assert_eq!(
        complete_write(attributes, PARTIALLY_FAILED_WRITE),
        [Ok(SENSITIVITY_ID), Err(CIE_ADDRESS_ID)],
    );
}

fn cie_address() -> Writable {
    Writable::IasCieAddress(IasCieAddress::new(IeeeAddress::default()))
}

fn complete_write<const N: usize>(
    attributes: [Writable; N],
    reply: impl IntoIterator<Item = u8>,
) -> Vec<WriteAttributeResult> {
    let frame = Frame::parse(IAS_ZONE_CLUSTER, reply.into_iter()).expect("valid write response");
    let (sender, messages) = channel(CAPACITY);
    let Poll::Ready(result) = complete_request(
        sender.write(destination(), endpoint(), attributes),
        messages,
        Ok(frame.into_payload()),
    ) else {
        panic!("expected the write to complete after the response");
    };
    result.expect("write response received")
}

fn complete_request<F, T>(
    future: F,
    mut messages: Receiver<Message>,
    reply: Result<Cluster, Error>,
) -> Poll<Result<T, Error>>
where
    F: Future<Output = Result<T, Error>>,
{
    let mut future = pin!(future);
    let mut context = Context::from_waker(Waker::noop());
    assert!(future.as_mut().poll(&mut context).is_pending());
    let Message::Communicate {
        request, response, ..
    } = messages.try_recv().expect("request queued")
    else {
        panic!("expected communication");
    };
    let (accepted, transmission) = oneshot::channel();
    let (received, protocol) = oneshot::channel::<Result<Cluster, Error>>();
    let (aps, _messages) = channel(CAPACITY);
    let key = ZclKey {
        short_id: ADDRESS,
        remote_endpoint: endpoint().get(),
        local_endpoint: endpoint().get(),
        cluster_id: request.cluster_id(),
        profile_id: request.profile_id(),
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
    received.send(reply).expect("response awaited");
    future.as_mut().poll(&mut context)
}
