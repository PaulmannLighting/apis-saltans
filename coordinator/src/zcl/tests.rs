use tokio::runtime::Builder;
use tokio::sync::mpsc::channel;
use zb_aps::TxOptions;
use zb_aps::apsde::{
    Alias, DataIndication, DataRequest, IndicationMetadata, IndicationStatus, IndividualEndpoint,
    NetworkAddress, ReceivedDestination, RequestDestination, Security, Source,
};
use zb_core::endpoint::Application;
use zb_core::{Cluster as ClusterId, Direction, Endpoint, Profile};
use zb_zcl::on_off::{Command as OnOffCommand, On};
use zb_zcl::{Cluster, Command, Frame, Header as ZclHeader, Scope, UnsequencedFrame};

use super::{Message, Subscription, SubscriptionFilter, SubscriptionMessage, Transceiver};
use crate::aps::Aps;
use crate::correlation::Key;
use crate::event::EventSink;
use crate::{Error, Event, MPSC_CHANNEL_SIZE};

const SOURCE_NODE_ID: u16 = 0x4321;
const TRANSACTION_SEQUENCE: u8 = 7;
const APS_COUNTER: u8 = 9;
const LINK_QUALITY: u8 = 255;
const LOCAL_NODE_ID: u16 = 0;
const LOCAL_ENDPOINT_ID: u8 = 11;
const REMOTE_ENDPOINT_ID: u8 = 12;
const RADIUS_COUNTER: u8 = 5;
const ALIAS_SEQUENCE_NUMBER: u8 = 6;

#[test]
fn encoding_preserves_every_aps_request_field() {
    let destination_address =
        NetworkAddress::new(SOURCE_NODE_ID).expect("test NWK address is valid");
    let alias_address =
        NetworkAddress::new(APS_COUNTER.into()).expect("test alias address is valid");
    let destination = RequestDestination::Network {
        address: destination_address,
        endpoint: Endpoint::try_from(REMOTE_ENDPOINT_ID).expect("remote endpoint is valid"),
    };
    let source_endpoint = IndividualEndpoint::new(
        Endpoint::try_from(LOCAL_ENDPOINT_ID).expect("local endpoint is valid"),
    )
    .expect("application endpoint is individual");
    let tx_options = TxOptions::SECURITY_ENABLED | TxOptions::ACKNOWLEDGED_TRANSMISSION;
    let alias = Alias::Use {
        source: alias_address,
        sequence_number: ALIAS_SEQUENCE_NUMBER,
    };
    let request = DataRequest::new(
        destination,
        Profile::ZigbeeHomeAutomation.as_u16(),
        ClusterId::OnOff.as_u16(),
        source_endpoint,
        UnsequencedFrame::from_command(On),
    )
    .with_tx_options(tx_options)
    .with_alias(alias)
    .with_radius_counter(RADIUS_COUNTER);

    let encoded = Transceiver::encode_request(request, TRANSACTION_SEQUENCE);
    let frame = Frame::parse(
        ClusterId::OnOff.as_u16(),
        encoded.asdu().clone().into_iter(),
    )
    .expect("encoded command is a valid ZCL frame");

    assert_eq!(encoded.destination(), destination);
    assert_eq!(encoded.profile_id(), Profile::ZigbeeHomeAutomation.as_u16());
    assert_eq!(encoded.cluster_id(), ClusterId::OnOff.as_u16());
    assert_eq!(encoded.source_endpoint(), source_endpoint);
    assert_eq!(encoded.tx_options(), tx_options);
    assert_eq!(encoded.alias(), alias);
    assert_eq!(encoded.radius_counter(), RADIUS_COUNTER);
    assert_eq!(frame.header().seq(), TRANSACTION_SEQUENCE);
    assert!(matches!(
        frame.payload(),
        Cluster::OnOff(OnOffCommand::On(_))
    ));
}

#[test]
fn communication_rejects_a_non_network_destination() {
    let source_endpoint = IndividualEndpoint::new(
        Endpoint::try_from(LOCAL_ENDPOINT_ID).expect("local endpoint is valid"),
    )
    .expect("application endpoint is individual");
    let request = DataRequest::new(
        RequestDestination::Bound,
        Profile::ZigbeeHomeAutomation.as_u16(),
        ClusterId::OnOff.as_u16(),
        source_endpoint,
        UnsequencedFrame::from_command(On),
    );

    assert!(matches!(
        Transceiver::request_key(&request, TRANSACTION_SEQUENCE),
        Err(Error::InvalidZclCommunicationDestination(
            RequestDestination::Bound
        ))
    ));
}

#[test]
fn response_free_unicast_rejects_an_enabled_default_response() {
    Builder::new_current_thread()
        .build()
        .expect("Tokio runtime")
        .block_on(async {
            let (mut transceiver, _events) = unstarted_transceiver();
            let destination = RequestDestination::Network {
                address: NetworkAddress::new(SOURCE_NODE_ID).expect("test NWK address is valid"),
                endpoint: Endpoint::try_from(REMOTE_ENDPOINT_ID).expect("remote endpoint is valid"),
            };
            let source_endpoint = IndividualEndpoint::new(
                Endpoint::try_from(LOCAL_ENDPOINT_ID).expect("local endpoint is valid"),
            )
            .expect("application endpoint is individual");
            let request = DataRequest::new(
                destination,
                Profile::ZigbeeHomeAutomation.as_u16(),
                ClusterId::OnOff.as_u16(),
                source_endpoint,
                UnsequencedFrame::from_command(On).with_disable_default_response(false),
            );

            assert!(matches!(
                transceiver.transmit(request).await,
                Err(Error::ZclDefaultResponseEnabled)
            ));
        });
}

#[test]
fn routes_matching_frames_to_a_generic_subscription() {
    Builder::new_current_thread()
        .build()
        .expect("Tokio runtime")
        .block_on(async {
            let (aps_sender, _aps_receiver) = channel(MPSC_CHANNEL_SIZE);
            let (events, mut application_events) = channel(MPSC_CHANNEL_SIZE);
            let filter = SubscriptionFilter::new(
                ClusterId::OnOff,
                Scope::ClusterSpecific,
                Direction::ClientToServer,
            );
            let (subscription, mut subscribed_frames) = Subscription::channel(filter);
            let (transceiver, messages) = channel(MPSC_CHANNEL_SIZE);
            tokio::spawn(
                Transceiver::new(
                    Aps::new(aps_sender),
                    EventSink::new(events),
                    transceiver.downgrade(),
                )
                .run(messages),
            );
            let source = source();

            transceiver
                .send(Message::Subscribe { subscription })
                .await
                .expect("ZCL transceiver remains available");
            transceiver
                .send(Message::Received {
                    indication: subscribed_indication(),
                })
                .await
                .expect("ZCL transceiver remains available");

            let received = subscribed_frames
                .recv()
                .await
                .expect("subscription remains open");
            assert_eq!(received.indication.metadata().source(), source);
            assert_eq!(received.indication.metadata().link_quality(), LINK_QUALITY);
            assert!(matches!(
                received.indication.asdu().payload(),
                Cluster::OnOff(OnOffCommand::On(_))
            ));
            assert!(application_events.try_recv().is_err());
        });
}

#[test]
fn matching_subscription_does_not_consume_correlated_response() {
    Builder::new_current_thread()
        .build()
        .expect("Tokio runtime")
        .block_on(async {
            let filter = SubscriptionFilter::new(
                ClusterId::OnOff,
                Scope::ClusterSpecific,
                Direction::ClientToServer,
            );
            let (subscription, mut subscribed_frames) = Subscription::channel(filter);
            let (mut transceiver, mut events) = unstarted_transceiver();
            transceiver.subscriptions.push(subscription);
            let indication = subscribed_indication();
            let response_key = Key::from_received_zcl_indication(&indication)
                .expect("test indication has a network source");
            let (_, _, response) = transceiver
                .responses
                .try_register_with_metadata(
                    |_| Ok(response_key),
                    super::ResponseExpectation::new::<On>(
                        <On as Command>::ID,
                        crate::DefaultResponsePolicy::SpecificRequired,
                    ),
                )
                .expect("response correlation can be registered");

            transceiver.handle_message_received(indication);

            assert!(matches!(
                response.await,
                Ok(Ok(Cluster::OnOff(OnOffCommand::On(_))))
            ));
            assert!(subscribed_frames.try_recv().is_err());
            assert!(events.try_recv().is_err());
        });
}

#[test]
fn subscription_request_does_not_complete_opposite_direction_response() {
    Builder::new_current_thread()
        .build()
        .expect("Tokio runtime")
        .block_on(async {
            let filter = SubscriptionFilter::new(
                ClusterId::OnOff,
                Scope::ClusterSpecific,
                Direction::ClientToServer,
            );
            let (subscription, mut subscribed_frames) = Subscription::channel(filter);
            let (mut transceiver, _events) = unstarted_transceiver();
            transceiver.subscriptions.push(subscription);
            let endpoint = Endpoint::Application(Application::MIN);
            let response_key = Key::new_zcl(
                SOURCE_NODE_ID,
                (endpoint, endpoint),
                ClusterId::OnOff.as_u16(),
                Profile::ZigbeeHomeAutomation.as_u16(),
                None,
                Direction::ServerToClient,
                TRANSACTION_SEQUENCE,
            );
            let (_, _, mut response) = transceiver
                .responses
                .try_register_with_metadata(
                    |_| Ok(response_key),
                    super::ResponseExpectation::new::<On>(
                        <On as Command>::ID,
                        crate::DefaultResponsePolicy::SpecificRequired,
                    ),
                )
                .expect("response correlation can be registered");

            transceiver.handle_message_received(subscribed_indication());

            assert!(subscribed_frames.recv().await.is_some());
            assert!(response.try_recv().is_err());
        });
}

#[test]
fn unregisters_a_subscription() {
    Builder::new_current_thread()
        .build()
        .expect("Tokio runtime")
        .block_on(async {
            let (aps_sender, _aps_receiver) = channel(MPSC_CHANNEL_SIZE);
            let (events, mut application_events) = channel(MPSC_CHANNEL_SIZE);
            let filter = SubscriptionFilter::new(
                ClusterId::OnOff,
                Scope::ClusterSpecific,
                Direction::ClientToServer,
            );
            let (subscription, subscribed_frames) = Subscription::channel(filter);
            let subscription_messages = subscribed_frames.sender();
            let (transceiver, messages) = channel(MPSC_CHANNEL_SIZE);
            tokio::spawn(
                Transceiver::new(
                    Aps::new(aps_sender),
                    EventSink::new(events),
                    transceiver.downgrade(),
                )
                .run(messages),
            );

            transceiver
                .send(Message::Subscribe { subscription })
                .await
                .expect("ZCL transceiver remains available");
            transceiver
                .send(Message::Unsubscribe {
                    messages: subscription_messages,
                })
                .await
                .expect("ZCL transceiver remains available");
            transceiver
                .send(Message::Received {
                    indication: subscribed_indication(),
                })
                .await
                .expect("ZCL transceiver remains available");

            let Some(Event::Zcl { indication }) = application_events.recv().await else {
                panic!("expected unmatched ZCL indication");
            };
            assert_eq!(indication.metadata().source(), source());
            assert_eq!(indication.metadata().link_quality(), LINK_QUALITY);
            assert!(matches!(
                indication.asdu().payload(),
                Cluster::OnOff(OnOffCommand::On(_))
            ));
        });
}

#[test]
fn removes_closed_subscriptions_during_delivery() {
    let (subscription, receiver) = Subscription::channel(SubscriptionFilter::new(
        ClusterId::OnOff,
        Scope::ClusterSpecific,
        Direction::ClientToServer,
    ));
    let (mut transceiver, _events) = unstarted_transceiver();
    transceiver.subscriptions.push(subscription);
    drop(receiver);

    let delivered = transceiver.forward_to_subscribers(&subscribed_indication());

    assert!(!delivered);
    assert!(transceiver.subscriptions.is_empty());
}

#[test]
fn full_subscription_does_not_block_normal_routing() {
    Builder::new_current_thread()
        .build()
        .expect("Tokio runtime")
        .block_on(async {
            let (subscription, _receiver) = Subscription::channel(SubscriptionFilter::new(
                ClusterId::OnOff,
                Scope::ClusterSpecific,
                Direction::ClientToServer,
            ));
            for _ in 0..MPSC_CHANNEL_SIZE {
                subscription
                    .try_send(SubscriptionMessage {
                        indication: subscribed_indication(),
                    })
                    .expect("subscription channel has capacity");
            }
            let (mut transceiver, mut events) = unstarted_transceiver();
            transceiver.subscriptions.push(subscription);

            transceiver.handle_message_received(subscribed_indication());

            assert!(matches!(events.try_recv(), Ok(Event::Zcl { .. })));
            assert_eq!(transceiver.subscriptions.len(), 1);
        });
}

fn unstarted_transceiver() -> (Transceiver, tokio::sync::mpsc::Receiver<Event>) {
    let (aps_sender, _aps_receiver) = channel(MPSC_CHANNEL_SIZE);
    let (events, application_events) = channel(MPSC_CHANNEL_SIZE);
    let (inbox, _messages) = channel(MPSC_CHANNEL_SIZE);
    (
        Transceiver::new(
            Aps::new(aps_sender),
            EventSink::new(events),
            inbox.downgrade(),
        ),
        application_events,
    )
}

fn subscribed_indication() -> DataIndication<Frame<Cluster>, (), ()> {
    let endpoint = IndividualEndpoint::new(Endpoint::Application(Application::MIN))
        .expect("application endpoint is individual");
    let metadata = IndicationMetadata::new(
        ReceivedDestination::Network {
            address: NetworkAddress::new(LOCAL_NODE_ID)
                .expect("coordinator address is a valid NWK address"),
            endpoint,
        },
        Source::Network {
            address: NetworkAddress::new(SOURCE_NODE_ID)
                .expect("source address is a valid NWK address"),
            endpoint,
        },
        Profile::ZigbeeHomeAutomation.as_u16(),
        ClusterId::OnOff.as_u16(),
        IndicationStatus::success(),
        Security::<()>::Unsecured,
        LINK_QUALITY,
        (),
    );
    let header = ZclHeader::new(
        Scope::ClusterSpecific,
        Direction::ClientToServer,
        false,
        None,
        TRANSACTION_SEQUENCE,
        <On as Command>::ID,
    );
    let frame = Frame::new(header, Cluster::OnOff(OnOffCommand::from(On)));
    DataIndication::new(metadata, frame)
}

fn source() -> Source {
    let endpoint = IndividualEndpoint::new(Endpoint::Application(Application::MIN))
        .expect("application endpoint is individual");
    Source::Network {
        address: NetworkAddress::new(SOURCE_NODE_ID)
            .expect("source address is a valid NWK address"),
        endpoint,
    }
}

#[test]
fn unrelated_reports_preserve_pending_responses_and_reach_normal_routing() {
    Builder::new_current_thread()
        .build()
        .expect("Tokio runtime")
        .block_on(async {
            for subscribe in [false, true] {
                let (mut transceiver, mut events) = unstarted_transceiver();
                let (subscription, mut reports) = Subscription::channel(SubscriptionFilter::new(
                    ClusterId::OnOff,
                    Scope::Global,
                    Direction::ServerToClient,
                ));
                if subscribe {
                    transceiver.subscriptions.push(subscription);
                }
                let response = read_response_indication();
                let key = Key::from_received_zcl_indication(&response).unwrap();
                let (_, _, mut receiver) =
                    transceiver
                        .responses
                        .try_register_with_metadata(
                            |_| Ok(key),
                            super::ResponseExpectation::new::<
                                zb_zcl::global::read_attributes::Response,
                            >(
                                <zb_zcl::global::read_attributes::Command as Command>::ID,
                                crate::DefaultResponsePolicy::SpecificRequired,
                            ),
                        )
                        .unwrap();
                let report = report_indication();
                assert_eq!(Key::from_received_zcl_indication(&report), Some(key));
                transceiver.handle_message_received(report);
                assert!(matches!(
                    receiver.try_recv(),
                    Err(tokio::sync::oneshot::error::TryRecvError::Empty)
                ));
                assert!(transceiver.responses.metadata(key).is_some());
                if subscribe {
                    assert!(reports.try_recv().is_ok());
                    assert!(events.try_recv().is_err());
                } else {
                    assert!(matches!(events.try_recv(), Ok(Event::Zcl { .. })));
                }
                transceiver.handle_message_received(response);
                assert!(matches!(
                    receiver.try_recv(),
                    Ok(Ok(Cluster::Global(
                        zb_zcl::global::Command::ReadAttributesResponse(_)
                    )))
                ));
                assert!(transceiver.responses.metadata(key).is_none());
            }
        });
}

#[test]
fn unrelated_reports_do_not_release_cancelled_or_timed_out_transactions() {
    Builder::new_current_thread()
        .build()
        .expect("Tokio runtime")
        .block_on(async {
            for timeout in [false, true] {
                let (mut transceiver, mut events) = unstarted_transceiver();
                let response = read_response_indication();
                let key = Key::from_received_zcl_indication(&response).unwrap();
                let (_, token, _receiver) =
                    transceiver
                        .responses
                        .try_register_with_metadata(
                            |_| Ok(key),
                            super::ResponseExpectation::new::<
                                zb_zcl::global::read_attributes::Response,
                            >(
                                <zb_zcl::global::read_attributes::Command as Command>::ID,
                                crate::DefaultResponsePolicy::SpecificRequired,
                            ),
                        )
                        .unwrap();
                if timeout {
                    assert!(transceiver.responses.timeout(token));
                } else {
                    assert!(transceiver.responses.cancel(token));
                }
                transceiver.handle_message_received(report_indication());
                assert!(transceiver.responses.metadata(key).is_some());
                assert!(matches!(events.try_recv(), Ok(Event::Zcl { .. })));
                transceiver.handle_message_received(response);
                assert!(transceiver.responses.metadata(key).is_none());
                assert!(events.try_recv().is_err());
            }
        });
}

#[test]
fn default_response_must_name_the_original_command() {
    const DEFAULT_RESPONSE: [u8; 5] = [0x18, TRANSACTION_SEQUENCE, 0x0b, 0x01, 0x00];
    const UNRELATED_DEFAULT_RESPONSE: [u8; 5] = [0x18, TRANSACTION_SEQUENCE, 0x0b, 0x00, 0x00];
    Builder::new_current_thread()
        .build()
        .expect("Tokio runtime")
        .block_on(async {
            for quarantined in [false, true] {
                let (mut transceiver, mut events) = unstarted_transceiver();
                let response = incoming_frame(&DEFAULT_RESPONSE);
                let unrelated = incoming_frame(&UNRELATED_DEFAULT_RESPONSE);
                let key = Key::from_received_zcl_indication(&response).unwrap();
                let (_, token, mut receiver) = transceiver
                    .responses
                    .try_register_with_metadata(
                        |_| Ok(key),
                        super::ResponseExpectation::new::<
                            zb_zcl::global::default_response::DefaultResponse,
                        >(
                            <On as Command>::ID,
                            crate::DefaultResponsePolicy::DefaultAllowed,
                        ),
                    )
                    .unwrap();
                if quarantined {
                    assert!(transceiver.responses.cancel(token));
                }
                transceiver.handle_message_received(unrelated);
                assert!(transceiver.responses.metadata(key).is_some());
                assert!(matches!(events.try_recv(), Ok(Event::Zcl { .. })));
                if !quarantined {
                    assert!(matches!(
                        receiver.try_recv(),
                        Err(tokio::sync::oneshot::error::TryRecvError::Empty)
                    ));
                }
                transceiver.handle_message_received(response);
                assert!(transceiver.responses.metadata(key).is_none());
                if !quarantined {
                    assert!(matches!(
                        receiver.try_recv(),
                        Ok(Ok(Cluster::Global(
                            zb_zcl::global::Command::DefaultResponse(_)
                        )))
                    ));
                }
                assert!(events.try_recv().is_err());
            }
        });
}

#[test]
fn local_endpoint_is_part_of_response_identity() {
    let local_endpoint =
        IndividualEndpoint::new(Endpoint::try_from(LOCAL_ENDPOINT_ID).unwrap()).unwrap();
    let remote_endpoint = IndividualEndpoint::new(Endpoint::Application(Application::MIN)).unwrap();
    let request = DataRequest::new(
        RequestDestination::Network {
            address: NetworkAddress::new(SOURCE_NODE_ID).unwrap(),
            endpoint: remote_endpoint.get(),
        },
        Profile::ZigbeeHomeAutomation.as_u16(),
        ClusterId::OnOff.as_u16(),
        local_endpoint,
        UnsequencedFrame::from_command(On),
    );
    let expected = Transceiver::request_key(&request, TRANSACTION_SEQUENCE).unwrap();
    let response = read_response_indication();
    assert_ne!(Key::from_received_zcl_indication(&response), Some(expected));
    let metadata = IndicationMetadata::new(
        ReceivedDestination::Network {
            address: NetworkAddress::new(LOCAL_NODE_ID).unwrap(),
            endpoint: local_endpoint,
        },
        source(),
        Profile::ZigbeeHomeAutomation.as_u16(),
        ClusterId::OnOff.as_u16(),
        IndicationStatus::success(),
        Security::<()>::Unsecured,
        LINK_QUALITY,
        (),
    );
    let corrected = DataIndication::new(metadata, response.into_parts().1);
    assert_eq!(
        Key::from_received_zcl_indication(&corrected),
        Some(expected)
    );
}

fn incoming_frame(bytes: &[u8]) -> DataIndication<Frame<Cluster>, (), ()> {
    subscribed_indication()
        .map_asdu(|_| Frame::parse(ClusterId::OnOff.as_u16(), bytes.iter().copied()).unwrap())
}

fn read_response_indication() -> DataIndication<Frame<Cluster>, (), ()> {
    const PAYLOAD: [u8; 8] = [
        0x18,
        TRANSACTION_SEQUENCE,
        0x01,
        0x00,
        0x00,
        0x00,
        0x10,
        0x01,
    ];
    incoming_frame(&PAYLOAD)
}

fn report_indication() -> DataIndication<Frame<Cluster>, (), ()> {
    const PAYLOAD: [u8; 7] = [0x18, TRANSACTION_SEQUENCE, 0x0a, 0x00, 0x00, 0x10, 0x01];
    incoming_frame(&PAYLOAD)
}

#[test]
fn error_default_responses_fail_typed_requests_and_release_quarantine() {
    const UNKNOWN_STATUS: u8 = 0xff;
    const REQUEST_ID: u8 = <zb_zcl::global::read_attributes::Command as Command>::ID;
    const DEFAULT_ID: u8 = <zb_zcl::global::default_response::DefaultResponse as Command>::ID;
    const FRAME_CONTROL: u8 = 0x18;
    Builder::new_current_thread()
        .build()
        .expect("Tokio runtime")
        .block_on(async {
            assert!(zb_zcl::Status::try_from(UNKNOWN_STATUS).is_err());
            for status in [zb_zcl::Status::Failure as u8, UNKNOWN_STATUS] {
                for quarantined in [false, true] {
                    let (mut transceiver, mut events) = unstarted_transceiver();
                    let response = incoming_frame(&[
                        FRAME_CONTROL,
                        TRANSACTION_SEQUENCE,
                        DEFAULT_ID,
                        REQUEST_ID,
                        status,
                    ]);
                    let key = Key::from_received_zcl_indication(&response).unwrap();
                    let (_, token, mut receiver) = transceiver
                        .responses
                        .try_register_with_metadata(
                            |_| Ok(key),
                            super::ResponseExpectation::new::<
                                zb_zcl::global::read_attributes::Response,
                            >(
                                REQUEST_ID, crate::DefaultResponsePolicy::SpecificRequired
                            ),
                        )
                        .unwrap();
                    if quarantined {
                        assert!(transceiver.responses.timeout(token));
                        assert!(matches!(
                            receiver.try_recv(),
                            Ok(Err(Error::ProtocolResponseTimeout))
                        ));
                    }
                    transceiver.handle_message_received(response);
                    assert!(transceiver.responses.metadata(key).is_none());
                    assert!(events.try_recv().is_err());
                    if !quarantined {
                        let Err(Error::Zcl(actual)) = receiver.try_recv().unwrap() else {
                            panic!("expected the device's ZCL error");
                        };
                        assert_eq!(actual, zb_zcl::Status::try_from(status));
                    }
                }
            }
        });
}

#[test]
fn unrelated_defaults_preserve_transactions_but_unexpected_success_resolves_them() {
    const SUCCESS: [u8; 5] = [0x18, TRANSACTION_SEQUENCE, 0x0b, 0x00, 0x00];
    const WRONG_COMMAND: [u8; 5] = [0x18, TRANSACTION_SEQUENCE, 0x0b, 0x01, 0x86];
    Builder::new_current_thread()
        .build()
        .expect("Tokio runtime")
        .block_on(async {
            for quarantined in [false, true] {
                let (mut transceiver, mut events) = unstarted_transceiver();
                let response = read_response_indication();
                let key = Key::from_received_zcl_indication(&response).unwrap();
                let (_, token, mut receiver) =
                    transceiver
                        .responses
                        .try_register_with_metadata(
                            |_| Ok(key),
                            super::ResponseExpectation::new::<
                                zb_zcl::global::read_attributes::Response,
                            >(
                                <zb_zcl::global::read_attributes::Command as Command>::ID,
                                crate::DefaultResponsePolicy::SpecificRequired,
                            ),
                        )
                        .unwrap();
                if quarantined {
                    assert!(transceiver.responses.cancel(token));
                }
                for bytes in [WRONG_COMMAND] {
                    transceiver.handle_message_received(incoming_frame(&bytes));
                    assert!(transceiver.responses.metadata(key).is_some());
                    assert!(matches!(events.try_recv(), Ok(Event::Zcl { .. })));
                    if !quarantined {
                        assert!(matches!(
                            receiver.try_recv(),
                            Err(tokio::sync::oneshot::error::TryRecvError::Empty)
                        ));
                    }
                }
                transceiver.handle_message_received(incoming_frame(&SUCCESS));
                assert!(transceiver.responses.metadata(key).is_none());
                assert!(events.try_recv().is_err());
                if !quarantined {
                    assert!(matches!(
                        receiver.try_recv(),
                        Ok(Err(Error::UnexpectedDefaultResponse { command_id: 0x00 }))
                    ));
                }
            }
        });
}

#[test]
fn allowed_defaults_resolve_specific_requests_for_all_statuses() {
    const FRAME_CONTROL: u8 = 0x18;
    const DEFAULT_ID: u8 = 0x0b;
    const REQUEST_ID: u8 = 0x00;
    const UNKNOWN_STATUS: u8 = 0xff;
    Builder::new_current_thread()
        .build()
        .expect("Tokio runtime")
        .block_on(async {
            for status in [
                zb_zcl::Status::Success as u8,
                zb_zcl::Status::Failure as u8,
                UNKNOWN_STATUS,
            ] {
                for quarantined in [false, true] {
                    let (mut transceiver, mut events) = unstarted_transceiver();
                    let response = incoming_frame(&[
                        FRAME_CONTROL,
                        TRANSACTION_SEQUENCE,
                        DEFAULT_ID,
                        REQUEST_ID,
                        status,
                    ]);
                    let key = Key::from_received_zcl_indication(&response).unwrap();
                    let (_, token, mut receiver) = transceiver
                        .responses
                        .try_register_with_metadata(
                            |_| Ok(key),
                            super::ResponseExpectation::new::<
                                zb_zcl::global::read_attributes::Response,
                            >(
                                REQUEST_ID, crate::DefaultResponsePolicy::DefaultAllowed
                            ),
                        )
                        .unwrap();
                    if quarantined {
                        assert!(transceiver.responses.cancel(token));
                    }
                    transceiver.handle_message_received(response);
                    assert!(transceiver.responses.metadata(key).is_none());
                    assert!(events.try_recv().is_err());
                    if !quarantined {
                        let raw = receiver.try_recv().unwrap().unwrap();
                        let crate::ZclOutcome::Default(response) = crate::ZclOutcome::<
                            zb_zcl::global::read_attributes::Response,
                        >::try_from(
                            raw
                        )
                        .unwrap() else {
                            panic!("expected a permitted Default Response");
                        };
                        assert_eq!(response.status(), status);
                    }
                }
            }
        });
}
