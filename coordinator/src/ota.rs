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
mod tests {
    use std::future::Future;
    use std::io::Cursor;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use std::time::Duration;

    use bytes::{BufMut, Bytes, BytesMut};
    use le_stream::{FromLeStream, ToLeStream};
    use tokio::time::timeout;
    use zb_aps::apsde::{
        DataIndication, IndicationMetadata, IndicationStatus, IndividualEndpoint, NetworkAddress,
        NetworkDestination, ReceivedDestination, Security, Source,
    };
    use zb_core::endpoint::Application;
    use zb_core::{Cluster, Direction, Endpoint, FullAddress, IeeeAddress, Profile, short_id};
    use zb_zcl::ota_upgrade::{
        Command as OtaCommand, ImageBlockRequest, ImageBlockResponse, ImageBlockResponsePayload,
        ImageId, ImageNotify, ImageNotifyPayload, ImagePageRequest, QueryNextImageRequest,
        QueryNextImageResponse, QueryResponse, QuerySpecificFileRequest, QuerySpecificFileResponse,
        UpgradeEndRequest, UpgradeEndResponse, UpgradeEndStatus,
    };
    use zb_zcl::{Cluster as ZclCluster, Command, Frame, Header, Scope};

    use super::{
        FieldControl, Image, Message, OTA_PROFILE, ParseImage, Request, Server, TEST_IEEE_ADDRESS,
        TransmissionResponse, UpdateError, UpdateResult, UpdateTimeouts,
    };
    use crate::{Error, Ota, zcl};

    const TEST_TIMEOUT: Duration = Duration::from_secs(1);
    const TEST_LIFECYCLE_TIMEOUT: Duration = Duration::from_millis(100);
    const LONG_LIFECYCLE_TIMEOUT: Duration = Duration::from_mins(1);
    const MANUFACTURER_CODE: u16 = 0x1234;
    const IMAGE_TYPE: u16 = 0x5678;
    const FILE_VERSION: u32 = 0x0102_0304;
    const STACK_VERSION: u16 = 0x0002;
    const OTA_FILE_IDENTIFIER: u32 = 0x0bee_f11e;
    const SUPPORTED_HEADER_VERSION: u16 = 0x0100;
    const BASE_HEADER_LENGTH: usize = 56;
    const HEADER_STRING_LENGTH: usize = 32;
    const UPGRADE_FILE_DESTINATION_LENGTH: usize = 8;
    const TEST_CHANNEL_SIZE: usize = 4;
    const TEST_SEQUENCE_NUMBER: u8 = 42;
    const TEST_APS_COUNTER: u8 = 1;
    const TEST_IMAGE_DATA: &[u8] = &[0xa5; 16];
    const PAGE_MAXIMUM_DATA_SIZE: u8 = 6;
    const PAGE_SIZE: u16 = 14;
    const PAGE_RESPONSE_SPACING: u16 = 0;
    const SINGLE_UPDATE_LIMIT: usize = 1;
    const TEST_UPDATE_LIMIT: usize = TEST_CHANNEL_SIZE;
    const LOCAL_NWK_ADDRESS: u16 = 0;
    const SECOND_DEVICE_SHORT_ID: u16 = 0x5678;
    const OTHER_IEEE_ADDRESS: IeeeAddress =
        IeeeAddress::new(0x00, 0x12, 0x4b, 0x00, 0x02, 0xdd, 0xee, 0xff);
    const ENDPOINT: Endpoint = Endpoint::Application(Application::MIN);

    enum ObservedZcl {
        Transmit {
            request: Request,
        },
        Reply {
            sequence_number: u8,
            request: Request,
        },
    }

    struct ScheduledUpdate {
        completion: tokio::sync::oneshot::Receiver<UpdateResult>,
        _cancellation: tokio::sync::oneshot::Sender<()>,
    }

    impl Future for ScheduledUpdate {
        type Output = Result<UpdateResult, tokio::sync::oneshot::error::RecvError>;

        fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
            Pin::new(&mut self.completion).poll(context)
        }
    }

    #[test]
    fn stops_when_external_ota_senders_are_dropped() {
        run_test(async {
            let (zcl_sender, _zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, server) = Server::test_new(zcl_sender, TEST_UPDATE_LIMIT);
            drop(ota_sender);

            timeout(TEST_TIMEOUT, server.run())
                .await
                .expect("OTA server did not stop after its inbox closed");
        });
    }

    #[test]
    fn active_subscription_does_not_keep_the_server_alive() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, server) = Server::test_new(zcl_sender, TEST_UPDATE_LIMIT);
            let server = tokio::spawn(server.run());
            let _completion = schedule(&ota_sender, test_image()).await;
            assert!(matches!(
                receive_raw_zcl(&mut zcl_receiver).await,
                zcl::Message::Subscribe { .. }
            ));

            drop(ota_sender);

            timeout(TEST_TIMEOUT, server)
                .await
                .expect("OTA server did not stop with an active subscription")
                .expect("OTA server task completed normally");
        });
    }

    #[test]
    fn scheduling_update_sends_unicast_image_notify() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, server) = Server::test_new(zcl_sender, TEST_UPDATE_LIMIT);
            tokio::spawn(server.run());
            let destination = test_destination();
            let (completion, _completion_result) = tokio::sync::oneshot::channel();
            let (_cancellation, cancelled) = tokio::sync::oneshot::channel();

            ota_sender
                .send(Message::Update {
                    target: test_address(),
                    target_endpoint: test_target_endpoint(),
                    source_endpoint: test_source_endpoint(),
                    image: test_image(),
                    timeouts: UpdateTimeouts::default(),
                    cancellation: cancelled,
                    completion,
                })
                .await
                .expect("OTA server is running");

            assert!(matches!(
                receive_raw_zcl(&mut zcl_receiver).await,
                zcl::Message::Subscribe { .. }
            ));
            let message = observe_zcl(receive_raw_zcl(&mut zcl_receiver).await);
            let ObservedZcl::Transmit { request } = message else {
                panic!("expected Image Notify transmission");
            };
            assert_eq!(request.destination(), destination.into());
            assert_eq!(request.profile_id(), OTA_PROFILE.as_u16());
            let (_, bytes) = request.into_asdu().into_parts();
            let notification =
                ImageNotify::from_le_stream(bytes.into_iter()).expect("valid Image Notify payload");
            assert!(matches!(
                notification.payload(),
                ImageNotifyPayload::FileVersion { image, .. }
                    if image.manufacturer_code() == MANUFACTURER_CODE
                        && image.image_type() == IMAGE_TYPE
                && image.file_version() == FILE_VERSION
            ));
        });
    }

    #[test]
    fn reports_a_subscription_registration_failure() {
        run_test(async {
            let (zcl_sender, zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, server) = Server::test_new(zcl_sender, TEST_UPDATE_LIMIT);
            drop(zcl_receiver);
            tokio::spawn(server.run());

            let result = ota_sender
                .update(
                    test_address(),
                    test_target_endpoint(),
                    test_source_endpoint(),
                    test_image(),
                )
                .await;

            assert!(matches!(result, Err(Error::Ota(UpdateError::Subscription))));
        });
    }

    #[test]
    fn reuses_the_subscription_for_a_replacement_update() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, server) = Server::test_new(zcl_sender, TEST_UPDATE_LIMIT);
            tokio::spawn(server.run());
            let previous_completion = schedule(&ota_sender, test_image()).await;

            assert!(matches!(
                receive_raw_zcl(&mut zcl_receiver).await,
                zcl::Message::Subscribe { .. }
            ));
            observe_zcl(receive_raw_zcl(&mut zcl_receiver).await);

            let replacement_completion = schedule(&ota_sender, test_image()).await;
            assert!(matches!(
                observe_zcl(receive_raw_zcl(&mut zcl_receiver).await),
                ObservedZcl::Transmit { .. }
            ));
            assert!(matches!(
                previous_completion.await,
                Ok(Err(UpdateError::Superseded))
            ));
            drop(replacement_completion);
        });
    }

    #[test]
    fn rejects_an_update_when_the_update_task_limit_is_reached() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, server) = Server::test_new(zcl_sender, SINGLE_UPDATE_LIMIT);
            tokio::spawn(server.run());
            let _first_completion = schedule(&ota_sender, test_image()).await;
            let _held_transmission = hold_next_transmission(&mut zcl_receiver).await;

            let result = ota_sender
                .update(
                    second_test_address(),
                    test_target_endpoint(),
                    test_source_endpoint(),
                    test_image(),
                )
                .await;

            assert!(matches!(
                result,
                Err(Error::Ota(UpdateError::UpdateTaskLimitReached {
                    limit: SINGLE_UPDATE_LIMIT
                }))
            ));
        });
    }

    #[test]
    fn dropping_update_future_releases_its_transfer_slot() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, server) = Server::test_new(zcl_sender, SINGLE_UPDATE_LIMIT);
            tokio::spawn(server.run());
            let first_update = update_via_api(ota_sender.clone(), test_image());
            receive_zcl(&mut zcl_receiver).await;

            first_update.abort();
            assert!(
                first_update
                    .await
                    .expect_err("the update task was aborted")
                    .is_cancelled()
            );
            assert!(matches!(
                timeout(TEST_TIMEOUT, receive_raw_zcl(&mut zcl_receiver))
                    .await
                    .expect("cancelled update did not unregister its subscription"),
                zcl::Message::Unsubscribe { .. }
            ));

            let second_update = update_via_api_for(ota_sender, second_test_address(), test_image());
            assert!(matches!(
                receive_raw_zcl(&mut zcl_receiver).await,
                zcl::Message::Subscribe { .. }
            ));
            let ObservedZcl::Transmit { request } = receive_zcl(&mut zcl_receiver).await else {
                panic!("expected Image Notify transmission");
            };
            assert_eq!(request.destination(), second_test_destination().into());
            second_update.abort();
        });
    }

    #[test]
    fn discovery_deadline_expires_an_unanswered_offer() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, server) = Server::test_new(zcl_sender, TEST_UPDATE_LIMIT);
            tokio::spawn(server.run());
            let timeouts = UpdateTimeouts::new(
                TEST_LIFECYCLE_TIMEOUT,
                LONG_LIFECYCLE_TIMEOUT,
                LONG_LIFECYCLE_TIMEOUT,
            );
            let completion =
                schedule_for(&ota_sender, test_address(), test_image(), timeouts).await;
            receive_zcl(&mut zcl_receiver).await;

            assert_eq!(
                timeout(TEST_TIMEOUT, completion)
                    .await
                    .expect("discovery deadline did not expire")
                    .expect("OTA completion sender was dropped"),
                Err(UpdateError::DiscoveryTimeout)
            );
        });
    }

    #[test]
    fn block_inactivity_deadline_resets_after_discovery() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, server) = Server::test_new(zcl_sender, TEST_UPDATE_LIMIT);
            tokio::spawn(server.run());
            let timeouts = UpdateTimeouts::new(
                LONG_LIFECYCLE_TIMEOUT,
                TEST_LIFECYCLE_TIMEOUT,
                LONG_LIFECYCLE_TIMEOUT,
            );
            let image = test_image();
            let current_image = ImageId::new(MANUFACTURER_CODE, IMAGE_TYPE, FILE_VERSION - 1);
            let completion = schedule_for(&ota_sender, test_address(), image, timeouts).await;
            receive_zcl(&mut zcl_receiver).await;

            ota_sender
                .send(incoming(
                    TEST_SEQUENCE_NUMBER,
                    QueryNextImageRequest::new(current_image, None),
                ))
                .await
                .expect("OTA server is running");
            receive_zcl(&mut zcl_receiver).await;

            assert_eq!(
                timeout(TEST_TIMEOUT, completion)
                    .await
                    .expect("block-inactivity deadline did not expire")
                    .expect("OTA completion sender was dropped"),
                Err(UpdateError::BlockInactivityTimeout)
            );
        });
    }

    #[test]
    fn total_transfer_deadline_bounds_the_complete_offer() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, server) = Server::test_new(zcl_sender, TEST_UPDATE_LIMIT);
            tokio::spawn(server.run());
            let timeouts = UpdateTimeouts::new(
                LONG_LIFECYCLE_TIMEOUT,
                LONG_LIFECYCLE_TIMEOUT,
                TEST_LIFECYCLE_TIMEOUT,
            );
            let completion =
                schedule_for(&ota_sender, test_address(), test_image(), timeouts).await;
            receive_zcl(&mut zcl_receiver).await;

            assert_eq!(
                timeout(TEST_TIMEOUT, completion)
                    .await
                    .expect("total-transfer deadline did not expire")
                    .expect("OTA completion sender was dropped"),
                Err(UpdateError::TotalTransferTimeout)
            );
        });
    }

    #[test]
    fn replaces_an_update_in_the_existing_destination_task() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, server) = Server::test_new(zcl_sender, SINGLE_UPDATE_LIMIT);
            tokio::spawn(server.run());
            let previous_completion = schedule(&ota_sender, test_image()).await;
            receive_zcl(&mut zcl_receiver).await;

            let replacement_completion = schedule(&ota_sender, test_image()).await;
            receive_zcl(&mut zcl_receiver).await;

            assert!(matches!(
                previous_completion.await,
                Ok(Err(UpdateError::Superseded))
            ));
            drop(replacement_completion);
        });
    }

    #[test]
    fn admits_a_new_destination_after_a_transfer_task_finishes() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, server) = Server::test_new(zcl_sender, SINGLE_UPDATE_LIMIT);
            tokio::spawn(server.run());
            let first_completion = schedule(&ota_sender, test_image()).await;
            fail_next_transmission(&mut zcl_receiver).await;
            assert!(matches!(
                first_completion.await,
                Ok(Err(UpdateError::Transmission))
            ));

            let _second_completion = schedule_for(
                &ota_sender,
                second_test_address(),
                test_image(),
                UpdateTimeouts::default(),
            )
            .await;
            let ObservedZcl::Transmit { request } = receive_zcl(&mut zcl_receiver).await else {
                panic!("expected Image Notify transmission");
            };
            assert_eq!(request.destination(), second_test_destination().into());
        });
    }

    #[test]
    fn unregisters_and_recreates_the_subscription_between_update_batches() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, server) = Server::test_new(zcl_sender, SINGLE_UPDATE_LIMIT);
            tokio::spawn(server.run());
            let first_completion = schedule(&ota_sender, test_image()).await;
            fail_next_transmission(&mut zcl_receiver).await;
            assert!(matches!(
                first_completion.await,
                Ok(Err(UpdateError::Transmission))
            ));
            assert!(matches!(
                receive_raw_zcl(&mut zcl_receiver).await,
                zcl::Message::Unsubscribe { .. }
            ));

            let _second_completion = schedule(&ota_sender, test_image()).await;
            assert!(matches!(
                receive_raw_zcl(&mut zcl_receiver).await,
                zcl::Message::Subscribe { .. }
            ));
        });
    }

    #[test]
    fn ignores_requests_outside_the_home_automation_profile() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, server) = Server::test_new(zcl_sender, TEST_UPDATE_LIMIT);
            tokio::spawn(server.run());
            let image = test_image();
            let _completion = schedule(&ota_sender, image).await;
            receive_zcl(&mut zcl_receiver).await;

            let current_image = ImageId::new(MANUFACTURER_CODE, IMAGE_TYPE, FILE_VERSION - 1);
            ota_sender
                .send(incoming_with_profile(
                    Profile::TouchLink,
                    TEST_SEQUENCE_NUMBER.wrapping_sub(1),
                    QueryNextImageRequest::new(current_image, None),
                ))
                .await
                .expect("OTA server is running");
            ota_sender
                .send(incoming(
                    TEST_SEQUENCE_NUMBER,
                    QueryNextImageRequest::new(current_image, None),
                ))
                .await
                .expect("OTA server is running");

            let (sequence_number, _) = reply_bytes(receive_zcl(&mut zcl_receiver).await);
            assert_eq!(sequence_number, TEST_SEQUENCE_NUMBER);
        });
    }

    #[test]
    fn routes_subscribed_frames_through_the_server_inbox() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, server) = Server::test_new(zcl_sender, TEST_UPDATE_LIMIT);
            tokio::spawn(server.run());
            let image = test_image();
            let _completion = schedule(&ota_sender, image).await;
            let zcl::Message::Subscribe { subscription } = receive_raw_zcl(&mut zcl_receiver).await
            else {
                panic!("expected OTA subscription registration");
            };
            receive_zcl(&mut zcl_receiver).await;

            let current_image = ImageId::new(MANUFACTURER_CODE, IMAGE_TYPE, FILE_VERSION - 1);
            subscription
                .try_send(subscribed(
                    TEST_SEQUENCE_NUMBER,
                    QueryNextImageRequest::new(current_image, None),
                ))
                .expect("OTA subscription remains available");

            let (sequence_number, bytes) = reply_bytes(receive_zcl(&mut zcl_receiver).await);
            assert_eq!(sequence_number, TEST_SEQUENCE_NUMBER);
            let response = QueryNextImageResponse::from_le_stream(bytes.into_iter())
                .expect("valid Query Next Image Response");
            assert!(matches!(response.response(), QueryResponse::Success { .. }));
        });
    }

    #[test]
    fn handles_query_block_and_upgrade_end_flow() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, server) = Server::test_new(zcl_sender, TEST_UPDATE_LIMIT);
            tokio::spawn(server.run());
            let image = test_image();
            let image_id = image.id();
            let image_size =
                u32::try_from(image.len()).expect("test image length fits OTA size field");
            let completion = update_via_api(ota_sender.clone(), image);
            receive_zcl(&mut zcl_receiver).await;

            let current_image = ImageId::new(MANUFACTURER_CODE, IMAGE_TYPE, FILE_VERSION - 1);
            ota_sender
                .send(incoming(
                    TEST_SEQUENCE_NUMBER,
                    QueryNextImageRequest::new(current_image, None),
                ))
                .await
                .expect("OTA server is running");
            let (sequence_number, bytes) = reply_bytes(receive_zcl(&mut zcl_receiver).await);
            assert_eq!(sequence_number, TEST_SEQUENCE_NUMBER);
            let response = QueryNextImageResponse::from_le_stream(bytes.into_iter())
                .expect("valid Query Next Image Response");
            assert_eq!(
                response.response(),
                QueryResponse::Success {
                    image: image_id,
                    image_size,
                }
            );

            let offset = u32::try_from(BASE_HEADER_LENGTH).expect("fixed header length fits u32");
            let maximum_data_size =
                u8::try_from(TEST_IMAGE_DATA.len()).expect("test block size fits u8");
            ota_sender
                .send(incoming(
                    TEST_SEQUENCE_NUMBER,
                    ImageBlockRequest::new(
                        image_id,
                        offset,
                        maximum_data_size,
                        Some(TEST_IEEE_ADDRESS),
                        None,
                    ),
                ))
                .await
                .expect("OTA server is running");
            let (sequence_number, bytes) = reply_bytes(receive_zcl(&mut zcl_receiver).await);
            assert_eq!(sequence_number, TEST_SEQUENCE_NUMBER);
            let response = ImageBlockResponse::from_le_stream(bytes.into_iter())
                .expect("valid Image Block Response");
            let ImageBlockResponsePayload::Success(block) = response.payload() else {
                panic!("expected a successful block response");
            };
            assert_eq!(block.file_offset(), offset);
            assert_eq!(block.image_data(), TEST_IMAGE_DATA);

            ota_sender
                .send(incoming(
                    TEST_SEQUENCE_NUMBER,
                    UpgradeEndRequest::new(UpgradeEndStatus::Success, image_id),
                ))
                .await
                .expect("OTA server is running");
            let (sequence_number, bytes) = reply_bytes(receive_zcl(&mut zcl_receiver).await);
            assert_eq!(sequence_number, TEST_SEQUENCE_NUMBER);
            let response = UpgradeEndResponse::from_le_stream(bytes.into_iter())
                .expect("valid Upgrade End Response");
            assert_eq!(response.image(), image_id);
            assert_eq!(response.current_time(), 0);
            assert_eq!(response.upgrade_time(), 0);
            assert!(matches!(completion.await, Ok(Ok(()))));
        });
    }

    #[test]
    fn serves_a_destination_restricted_image_to_its_pinned_identity() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, server) = Server::test_new(zcl_sender, TEST_UPDATE_LIMIT);
            tokio::spawn(server.run());
            let image = test_image_for(Some(TEST_IEEE_ADDRESS));
            let image_id = image.id();
            let _completion = schedule(&ota_sender, image).await;
            receive_zcl(&mut zcl_receiver).await;

            ota_sender
                .send(incoming(
                    TEST_SEQUENCE_NUMBER,
                    QuerySpecificFileRequest::new(TEST_IEEE_ADDRESS, image_id, STACK_VERSION),
                ))
                .await
                .expect("OTA server is running");

            let (sequence_number, bytes) = reply_bytes(receive_zcl(&mut zcl_receiver).await);
            assert_eq!(sequence_number, TEST_SEQUENCE_NUMBER);
            let response = QuerySpecificFileResponse::from_le_stream(bytes.into_iter())
                .expect("valid Query Specific File Response");
            assert!(matches!(response.response(), QueryResponse::Success { .. }));

            let offset = u32::try_from(BASE_HEADER_LENGTH + UPGRADE_FILE_DESTINATION_LENGTH)
                .expect("test payload offset fits u32");
            let maximum_data_size =
                u8::try_from(TEST_IMAGE_DATA.len()).expect("test block size fits u8");
            ota_sender
                .send(incoming(
                    TEST_SEQUENCE_NUMBER,
                    ImageBlockRequest::new(
                        image_id,
                        offset,
                        maximum_data_size,
                        Some(TEST_IEEE_ADDRESS),
                        None,
                    ),
                ))
                .await
                .expect("OTA server is running");

            let (_, bytes) = reply_bytes(receive_zcl(&mut zcl_receiver).await);
            let response = ImageBlockResponse::from_le_stream(bytes.into_iter())
                .expect("valid Image Block Response");
            assert!(matches!(
                response.payload(),
                ImageBlockResponsePayload::Success(_)
            ));
        });
    }

    #[test]
    fn rejects_a_request_when_the_short_address_resolves_to_another_identity() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, server) =
                Server::test_new_resolving(zcl_sender, TEST_UPDATE_LIMIT, OTHER_IEEE_ADDRESS);
            tokio::spawn(server.run());
            let image = test_image();
            let current_image = ImageId::new(MANUFACTURER_CODE, IMAGE_TYPE, FILE_VERSION - 1);
            let _completion = schedule(&ota_sender, image).await;
            receive_zcl(&mut zcl_receiver).await;

            ota_sender
                .send(incoming(
                    TEST_SEQUENCE_NUMBER,
                    QueryNextImageRequest::new(current_image, None),
                ))
                .await
                .expect("OTA server is running");

            let (_, bytes) = reply_bytes(receive_zcl(&mut zcl_receiver).await);
            let response = QueryNextImageResponse::from_le_stream(bytes.into_iter())
                .expect("valid Query Next Image Response");
            assert_eq!(response.response(), QueryResponse::NotAuthorized);
        });
    }

    #[test]
    fn update_reports_a_background_transmission_failure() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, server) = Server::test_new(zcl_sender, TEST_UPDATE_LIMIT);
            tokio::spawn(server.run());
            let completion = update_via_api(ota_sender, test_image());

            fail_next_transmission(&mut zcl_receiver).await;

            let result = timeout(TEST_TIMEOUT, completion)
                .await
                .expect("OTA completion timed out")
                .expect("OTA update task completed normally");
            assert!(matches!(result, Err(Error::Ota(UpdateError::Transmission))));
        });
    }

    #[test]
    fn hardware_unavailability_fails_an_active_update_and_stops_the_server() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, server) = Server::test_new(zcl_sender, TEST_UPDATE_LIMIT);
            let server = tokio::spawn(server.run());
            let completion = update_via_api(ota_sender.clone(), test_image());
            receive_zcl(&mut zcl_receiver).await;

            ota_sender
                .send(Message::HardwareUnavailable)
                .await
                .expect("OTA server is running");

            let result = timeout(TEST_TIMEOUT, completion)
                .await
                .expect("OTA completion timed out")
                .expect("OTA update task completed normally");
            assert!(matches!(
                result,
                Err(Error::Ota(UpdateError::HardwareEventStreamClosed))
            ));
            timeout(TEST_TIMEOUT, server)
                .await
                .expect("OTA server did not stop after hardware became unavailable")
                .expect("OTA server task completed normally");
        });
    }

    #[test]
    fn update_reports_the_clients_terminal_failure() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, server) = Server::test_new(zcl_sender, TEST_UPDATE_LIMIT);
            tokio::spawn(server.run());
            let image = test_image();
            let image_id = image.id();
            let completion = update_via_api(ota_sender.clone(), image);
            receive_zcl(&mut zcl_receiver).await;

            ota_sender
                .send(incoming(
                    TEST_SEQUENCE_NUMBER,
                    UpgradeEndRequest::new(UpgradeEndStatus::InvalidImage, image_id),
                ))
                .await
                .expect("OTA server is running");
            receive_zcl(&mut zcl_receiver).await;

            let result = timeout(TEST_TIMEOUT, completion)
                .await
                .expect("OTA completion timed out")
                .expect("OTA update task completed normally");
            assert!(matches!(result, Err(Error::Ota(UpdateError::InvalidImage))));
        });
    }

    #[test]
    fn image_page_uses_consecutive_transaction_sequence_numbers() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, server) = Server::test_new(zcl_sender, TEST_UPDATE_LIMIT);
            tokio::spawn(server.run());
            let image = test_image();
            let image_id = image.id();
            let _completion = schedule(&ota_sender, image).await;
            receive_zcl(&mut zcl_receiver).await;

            ota_sender
                .send(incoming(
                    TEST_SEQUENCE_NUMBER,
                    ImagePageRequest::new(
                        image_id,
                        u32::try_from(BASE_HEADER_LENGTH).expect("fixed header length fits u32"),
                        PAGE_MAXIMUM_DATA_SIZE,
                        PAGE_SIZE,
                        PAGE_RESPONSE_SPACING,
                        None,
                    ),
                ))
                .await
                .expect("OTA server is running");

            for index in 0..3 {
                let (sequence_number, tx_options, bytes) =
                    reply_parts(receive_zcl(&mut zcl_receiver).await);
                assert_eq!(sequence_number, TEST_SEQUENCE_NUMBER.wrapping_add(index));
                assert!(tx_options.is_empty());
                let response = ImageBlockResponse::from_le_stream(bytes.into_iter())
                    .expect("valid Image Block Response");
                assert!(matches!(
                    response.payload(),
                    ImageBlockResponsePayload::Success(_)
                ));
            }
        });
    }

    fn test_image() -> Image {
        test_image_for(None)
    }

    fn test_image_for(destination: Option<IeeeAddress>) -> Image {
        let optional_header_length = destination.map_or(0, |_| UPGRADE_FILE_DESTINATION_LENGTH);
        let header_length = BASE_HEADER_LENGTH + optional_header_length;
        let total_length = header_length + TEST_IMAGE_DATA.len();
        let mut bytes = BytesMut::with_capacity(total_length);
        bytes.put_u32_le(OTA_FILE_IDENTIFIER);
        bytes.put_u16_le(SUPPORTED_HEADER_VERSION);
        bytes.put_u16_le(u16::try_from(header_length).expect("test header length fits u16"));
        let field_control = destination.map_or(FieldControl::empty(), |_| {
            FieldControl::UPGRADE_FILE_DESTINATION
        });
        bytes.put_u16_le(field_control.bits());
        bytes.put_u16_le(MANUFACTURER_CODE);
        bytes.put_u16_le(IMAGE_TYPE);
        bytes.put_u32_le(FILE_VERSION);
        bytes.put_u16_le(STACK_VERSION);
        bytes.extend_from_slice(&[0; HEADER_STRING_LENGTH]);
        bytes.put_u32_le(u32::try_from(total_length).expect("test image length fits u32"));
        bytes.extend(destination.to_le_stream());
        bytes.extend_from_slice(TEST_IMAGE_DATA);
        Cursor::new(bytes.freeze())
            .parse()
            .expect("valid test image")
    }

    fn test_address() -> FullAddress {
        FullAddress::new(TEST_IEEE_ADDRESS, test_short_id())
    }

    fn second_test_address() -> FullAddress {
        FullAddress::new(TEST_IEEE_ADDRESS, second_test_short_id())
    }

    fn test_short_id() -> short_id::Device {
        short_id::Device::new(0x1234).expect("valid short ID")
    }

    fn second_test_short_id() -> short_id::Device {
        short_id::Device::new(SECOND_DEVICE_SHORT_ID).expect("valid short ID")
    }

    fn test_destination() -> NetworkDestination {
        NetworkDestination::new(
            NetworkAddress::new(test_short_id().as_u16()).expect("valid NWK address"),
            test_target_endpoint(),
        )
    }

    fn second_test_destination() -> NetworkDestination {
        NetworkDestination::new(
            NetworkAddress::new(second_test_short_id().as_u16()).expect("valid NWK address"),
            test_target_endpoint(),
        )
    }

    const fn test_source_endpoint() -> IndividualEndpoint {
        IndividualEndpoint::new(ENDPOINT).expect("test endpoint is individual")
    }

    const fn test_target_endpoint() -> IndividualEndpoint {
        IndividualEndpoint::new(ENDPOINT).expect("test endpoint is individual")
    }

    async fn schedule(
        sender: &tokio::sync::mpsc::Sender<Message>,
        image: Image,
    ) -> ScheduledUpdate {
        schedule_for(sender, test_address(), image, UpdateTimeouts::default()).await
    }

    async fn schedule_for(
        sender: &tokio::sync::mpsc::Sender<Message>,
        target: FullAddress,
        image: Image,
        timeouts: UpdateTimeouts,
    ) -> ScheduledUpdate {
        let (completion, result) = tokio::sync::oneshot::channel();
        let (cancellation, cancelled) = tokio::sync::oneshot::channel();
        sender
            .send(Message::Update {
                target,
                target_endpoint: test_target_endpoint(),
                source_endpoint: test_source_endpoint(),
                image,
                timeouts,
                cancellation: cancelled,
                completion,
            })
            .await
            .expect("OTA server is running");
        ScheduledUpdate {
            completion: result,
            _cancellation: cancellation,
        }
    }

    fn update_via_api(
        sender: tokio::sync::mpsc::Sender<Message>,
        image: Image,
    ) -> tokio::task::JoinHandle<Result<(), Error>> {
        update_via_api_for(sender, test_address(), image)
    }

    fn update_via_api_for(
        sender: tokio::sync::mpsc::Sender<Message>,
        target: FullAddress,
        image: Image,
    ) -> tokio::task::JoinHandle<Result<(), Error>> {
        tokio::spawn(async move {
            sender
                .update(
                    target,
                    test_target_endpoint(),
                    test_source_endpoint(),
                    image,
                )
                .await
        })
    }

    fn incoming<T>(sequence_number: u8, command: T) -> Message
    where
        T: Command + Into<OtaCommand>,
    {
        incoming_with_profile(OTA_PROFILE, sequence_number, command)
    }

    fn incoming_with_profile<T>(profile: Profile, sequence_number: u8, command: T) -> Message
    where
        T: Command + Into<OtaCommand>,
    {
        let zcl_header = Header::new(
            Scope::ClusterSpecific,
            Direction::ClientToServer,
            false,
            None,
            sequence_number,
            T::ID,
        );
        Message::Received {
            indication: DataIndication::new(
                test_metadata(profile),
                Frame::new(zcl_header, command.into()),
            ),
        }
    }

    fn subscribed<T>(sequence_number: u8, command: T) -> zcl::SubscriptionMessage
    where
        T: Command + Into<OtaCommand>,
    {
        let zcl_header = Header::new(
            Scope::ClusterSpecific,
            Direction::ClientToServer,
            false,
            None,
            sequence_number,
            T::ID,
        );
        zcl::SubscriptionMessage {
            indication: DataIndication::new(
                test_metadata(OTA_PROFILE),
                Frame::new(zcl_header, ZclCluster::OtaUpgrade(command.into())),
            ),
        }
    }

    fn test_metadata(profile: Profile) -> IndicationMetadata<(), ()> {
        let endpoint =
            IndividualEndpoint::new(ENDPOINT).expect("test endpoint is an individual endpoint");
        IndicationMetadata::new(
            ReceivedDestination::Network {
                address: NetworkAddress::new(LOCAL_NWK_ADDRESS)
                    .expect("coordinator address is valid"),
                endpoint,
            },
            test_source(),
            profile.as_u16(),
            Cluster::OtaUpgrade.as_u16(),
            IndicationStatus::success(),
            Security::Unsecured,
            u8::MAX,
            (),
        )
    }

    fn test_source() -> Source {
        Source::Network {
            address: test_destination().address(),
            endpoint: IndividualEndpoint::new(ENDPOINT)
                .expect("test endpoint is an individual endpoint"),
        }
    }

    async fn receive_zcl(receiver: &mut tokio::sync::mpsc::Receiver<zcl::Message>) -> ObservedZcl {
        loop {
            let message = receive_raw_zcl(receiver).await;
            if matches!(
                message,
                zcl::Message::Subscribe { .. } | zcl::Message::Unsubscribe { .. }
            ) {
                continue;
            }
            return observe_zcl(message);
        }
    }

    async fn receive_raw_zcl(
        receiver: &mut tokio::sync::mpsc::Receiver<zcl::Message>,
    ) -> zcl::Message {
        timeout(TEST_TIMEOUT, receiver.recv())
            .await
            .expect("OTA server response timed out")
            .expect("ZCL actor channel is open")
    }

    fn observe_zcl(message: zcl::Message) -> ObservedZcl {
        match message {
            zcl::Message::Transmit { request, response } => {
                complete_transmission(response);
                ObservedZcl::Transmit { request }
            }
            zcl::Message::Reply {
                sequence_number,
                request,
                response,
            } => {
                complete_transmission(response);
                ObservedZcl::Reply {
                    sequence_number,
                    request,
                }
            }
            other => panic!("unexpected ZCL message: {other:?}"),
        }
    }

    async fn fail_next_transmission(receiver: &mut tokio::sync::mpsc::Receiver<zcl::Message>) {
        let message = receive_non_subscription(receiver).await;
        let zcl::Message::Transmit { response, .. } = message else {
            panic!("expected OTA transmission");
        };
        let (completion, transmission) = deferred_transmission();
        assert!(
            completion
                .send(Err(zb_hw::Error::Unsupported(zb_hw::Operation::Transmit)))
                .is_ok()
        );
        assert!(response.send(Ok(transmission)).is_ok());
    }

    async fn hold_next_transmission(
        receiver: &mut tokio::sync::mpsc::Receiver<zcl::Message>,
    ) -> tokio::sync::oneshot::Sender<Result<(), zb_hw::Error>> {
        let message = receive_non_subscription(receiver).await;
        let zcl::Message::Transmit { response, .. } = message else {
            panic!("expected OTA transmission");
        };
        let (completion, transmission) = deferred_transmission();
        assert!(response.send(Ok(transmission)).is_ok());
        completion
    }

    async fn receive_non_subscription(
        receiver: &mut tokio::sync::mpsc::Receiver<zcl::Message>,
    ) -> zcl::Message {
        loop {
            let message = receive_raw_zcl(receiver).await;
            if !matches!(
                message,
                zcl::Message::Subscribe { .. } | zcl::Message::Unsubscribe { .. }
            ) {
                return message;
            }
        }
    }

    fn reply_bytes(message: ObservedZcl) -> (u8, Bytes) {
        let (sequence_number, tx_options, bytes) = reply_parts(message);
        assert_eq!(tx_options, zb_aps::TxOptions::ACKNOWLEDGED_TRANSMISSION);
        (sequence_number, bytes)
    }

    fn reply_parts(message: ObservedZcl) -> (u8, zb_aps::TxOptions, Bytes) {
        let ObservedZcl::Reply {
            sequence_number,
            request,
        } = message
        else {
            panic!("expected OTA reply");
        };
        assert_eq!(request.destination(), test_destination().into());
        let tx_options = request.tx_options();
        let (_, bytes) = request.into_asdu().into_parts();
        (sequence_number, tx_options, bytes)
    }

    fn complete_transmission(
        response: tokio::sync::oneshot::Sender<Result<TransmissionResponse, Error>>,
    ) {
        let (completion, transmission) = deferred_transmission();
        assert!(completion.send(Ok(())).is_ok());
        assert!(response.send(Ok(transmission)).is_ok());
    }

    fn deferred_transmission() -> (
        tokio::sync::oneshot::Sender<Result<(), zb_hw::Error>>,
        TransmissionResponse,
    ) {
        let (completion, result) = tokio::sync::oneshot::channel();
        let (inbox, _messages) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
        (
            completion,
            TransmissionResponse::test_new(result, TEST_APS_COUNTER, inbox.downgrade()),
        )
    }

    fn run_test<T>(future: T)
    where
        T: Future<Output = ()>,
    {
        tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .expect("Tokio runtime")
            .block_on(future);
    }
}
