//! Coordinator-owned OTA Upgrade server.

use log::warn;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use zb_core::Profile;
use zb_core::destination::Device;
use zb_hw::HwResponse;

pub use self::image::{
    BaseHeaderBytes, FieldControl, Header, HeaderString, Image, ParseImage, ParseImageError,
};
pub use self::message::{Message, UpdateError, UpdateResult};
pub use self::server::Server;
use crate::zcl::{self, Metadata, Payload};

mod image;
mod message;
mod page_transfer;
mod server;
mod state;
mod transfer;

const CURRENT_TIME_IMMEDIATE: u32 = 0;
const UPGRADE_TIME_IMMEDIATE: u32 = 0;
const OTA_PROFILE: Profile = Profile::ZigbeeHomeAutomation;

async fn reply_zcl(
    zcl: &Sender<zcl::Message>,
    destination: Device,
    profile: Profile,
    sequence_number: u8,
    payload: Payload,
) -> Option<HwResponse> {
    let (response, result) = oneshot::channel();
    if let Err(error) = zcl
        .send(zcl::Message::Reply {
            destination,
            sequence_number,
            payload: payload.with_profile(profile),
            response,
        })
        .await
    {
        warn!("Failed to queue OTA reply: {error}");
        return None;
    }
    receive_hw_response(result).await
}

async fn send_zcl(
    zcl: &Sender<zcl::Message>,
    destination: zb_core::Destination,
    payload: Payload,
) -> Option<HwResponse> {
    let (response, result) = oneshot::channel();
    if let Err(error) = zcl
        .send(zcl::Message::Transmit {
            destination,
            payload,
            response,
        })
        .await
    {
        warn!("Failed to queue OTA command: {error}");
        return None;
    }
    receive_hw_response(result).await
}

async fn receive_hw_response(
    response: oneshot::Receiver<Result<HwResponse, zb_hw::Error>>,
) -> Option<HwResponse> {
    match response.await {
        Ok(Ok(response)) => Some(response),
        Ok(Err(error)) => {
            warn!("Failed to start OTA transmission: {error}");
            None
        }
        Err(error) => {
            warn!("Failed to receive OTA hardware response: {error}");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::io::Cursor;
    use std::time::Duration;

    use bytes::{BufMut, Bytes, BytesMut};
    use le_stream::FromLeStream;
    use tokio::time::timeout;
    use zb_aps::Data;
    use zb_core::destination::Device;
    use zb_core::endpoint::Application;
    use zb_core::{Cluster, Direction, Endpoint, Profile, short_id};
    use zb_hw::HwResponse;
    use zb_nwk::Source;
    use zb_zcl::ota_upgrade::{
        Command as OtaCommand, ImageBlockRequest, ImageBlockResponse, ImageBlockResponsePayload,
        ImageId, ImageNotify, ImageNotifyPayload, ImagePageRequest, QueryNextImageRequest,
        QueryNextImageResponse, QueryResponse, UpgradeEndRequest, UpgradeEndResponse,
        UpgradeEndStatus,
    };
    use zb_zcl::{Command, Frame, Header, Scope};

    use super::{Image, Message, OTA_PROFILE, ParseImage, Server, UpdateError, UpdateResult};
    use crate::zcl::{self, Payload};
    use crate::{Error, Ota};

    const TEST_TIMEOUT: Duration = Duration::from_secs(1);
    const MANUFACTURER_CODE: u16 = 0x1234;
    const IMAGE_TYPE: u16 = 0x5678;
    const FILE_VERSION: u32 = 0x0102_0304;
    const STACK_VERSION: u16 = 0x0002;
    const OTA_FILE_IDENTIFIER: u32 = 0x0bee_f11e;
    const SUPPORTED_HEADER_VERSION: u16 = 0x0100;
    const BASE_HEADER_LENGTH: usize = 56;
    const HEADER_STRING_LENGTH: usize = 32;
    const TEST_CHANNEL_SIZE: usize = 4;
    const TEST_SEQUENCE_NUMBER: u8 = 42;
    const TEST_IMAGE_DATA: &[u8] = &[0xa5; 16];
    const PAGE_MAXIMUM_DATA_SIZE: u8 = 6;
    const PAGE_SIZE: u16 = 14;
    const PAGE_RESPONSE_SPACING: u16 = 0;
    const SINGLE_UPDATE_LIMIT: usize = 1;
    const TEST_UPDATE_LIMIT: usize = TEST_CHANNEL_SIZE;
    const SECOND_DEVICE_SHORT_ID: u16 = 0x5678;
    const ENDPOINT: Endpoint = Endpoint::Application(Application::MIN);

    enum ObservedZcl {
        Transmit {
            destination: zb_core::Destination,
            payload: Payload,
        },
        Reply {
            destination: Device,
            sequence_number: u8,
            payload: Payload,
        },
    }

    #[test]
    fn stops_when_external_ota_senders_are_dropped() {
        run_test(async {
            let (zcl_sender, _zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, ota_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let server = Server::test_new(zcl_sender, ota_receiver, TEST_UPDATE_LIMIT);
            drop(ota_sender);

            timeout(TEST_TIMEOUT, server.run())
                .await
                .expect("OTA server did not stop after its inbox closed");
        });
    }

    #[test]
    fn scheduling_update_sends_unicast_image_notify() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, ota_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            tokio::spawn(Server::test_new(zcl_sender, ota_receiver, TEST_UPDATE_LIMIT).run());
            let destination = test_destination();
            let (completion, _completion_result) = tokio::sync::oneshot::channel();

            ota_sender
                .send(Message::Update {
                    target: destination,
                    image: test_image(),
                    completion,
                })
                .await
                .expect("OTA server is running");

            let message = receive_zcl(&mut zcl_receiver).await;
            let ObservedZcl::Transmit {
                destination: actual_destination,
                payload,
            } = message
            else {
                panic!("expected Image Notify transmission");
            };
            assert_eq!(actual_destination, destination.into());
            let (metadata, _, bytes) = payload.into_parts();
            assert_eq!(metadata.profile(), OTA_PROFILE);
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
    fn rejects_an_update_when_the_update_task_limit_is_reached() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, ota_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            tokio::spawn(Server::test_new(zcl_sender, ota_receiver, SINGLE_UPDATE_LIMIT).run());
            let _first_completion = schedule(&ota_sender, test_image()).await;
            hold_next_transmission(&mut zcl_receiver).await;

            let result = ota_sender
                .update(second_test_destination(), test_image())
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
    fn replaces_an_update_in_the_existing_destination_task() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, ota_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            tokio::spawn(Server::test_new(zcl_sender, ota_receiver, SINGLE_UPDATE_LIMIT).run());
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
            let (ota_sender, ota_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            tokio::spawn(Server::test_new(zcl_sender, ota_receiver, SINGLE_UPDATE_LIMIT).run());
            let first_completion = schedule(&ota_sender, test_image()).await;
            fail_next_transmission(&mut zcl_receiver).await;
            assert!(matches!(
                first_completion.await,
                Ok(Err(UpdateError::Transmission))
            ));

            let _second_completion =
                schedule_for(&ota_sender, second_test_destination(), test_image()).await;
            let ObservedZcl::Transmit { destination, .. } = receive_zcl(&mut zcl_receiver).await
            else {
                panic!("expected Image Notify transmission");
            };
            assert_eq!(destination, second_test_destination().into());
        });
    }

    #[test]
    fn ignores_requests_outside_the_home_automation_profile() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, ota_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            tokio::spawn(Server::test_new(zcl_sender, ota_receiver, TEST_UPDATE_LIMIT).run());
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
    fn handles_query_block_and_upgrade_end_flow() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, ota_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            tokio::spawn(Server::test_new(zcl_sender, ota_receiver, TEST_UPDATE_LIMIT).run());
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
                    ImageBlockRequest::new(image_id, offset, maximum_data_size, None, None),
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
    fn update_reports_a_background_transmission_failure() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, ota_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            tokio::spawn(Server::test_new(zcl_sender, ota_receiver, TEST_UPDATE_LIMIT).run());
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
    fn update_reports_the_clients_terminal_failure() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, ota_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            tokio::spawn(Server::test_new(zcl_sender, ota_receiver, TEST_UPDATE_LIMIT).run());
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
            let (ota_sender, ota_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            tokio::spawn(Server::test_new(zcl_sender, ota_receiver, TEST_UPDATE_LIMIT).run());
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
                let (sequence_number, metadata, bytes) =
                    reply_parts(receive_zcl(&mut zcl_receiver).await);
                assert_eq!(sequence_number, TEST_SEQUENCE_NUMBER.wrapping_add(index));
                assert!(metadata.tx_options().is_empty());
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
        let total_length = BASE_HEADER_LENGTH + TEST_IMAGE_DATA.len();
        let mut bytes = BytesMut::with_capacity(total_length);
        bytes.put_u32_le(OTA_FILE_IDENTIFIER);
        bytes.put_u16_le(SUPPORTED_HEADER_VERSION);
        bytes.put_u16_le(u16::try_from(BASE_HEADER_LENGTH).expect("fixed header length fits u16"));
        bytes.put_u16_le(0);
        bytes.put_u16_le(MANUFACTURER_CODE);
        bytes.put_u16_le(IMAGE_TYPE);
        bytes.put_u32_le(FILE_VERSION);
        bytes.put_u16_le(STACK_VERSION);
        bytes.extend_from_slice(&[0; HEADER_STRING_LENGTH]);
        bytes.put_u32_le(u32::try_from(total_length).expect("test image length fits u32"));
        bytes.extend_from_slice(TEST_IMAGE_DATA);
        Cursor::new(bytes.freeze())
            .parse()
            .expect("valid test image")
    }

    fn test_destination() -> Device {
        Device::new(
            short_id::Device::new(0x1234).expect("valid short ID"),
            ENDPOINT,
        )
    }

    fn second_test_destination() -> Device {
        Device::new(
            short_id::Device::new(SECOND_DEVICE_SHORT_ID).expect("valid short ID"),
            ENDPOINT,
        )
    }

    async fn schedule(
        sender: &tokio::sync::mpsc::Sender<Message>,
        image: Image,
    ) -> tokio::sync::oneshot::Receiver<UpdateResult> {
        schedule_for(sender, test_destination(), image).await
    }

    async fn schedule_for(
        sender: &tokio::sync::mpsc::Sender<Message>,
        target: Device,
        image: Image,
    ) -> tokio::sync::oneshot::Receiver<UpdateResult> {
        let (completion, result) = tokio::sync::oneshot::channel();
        sender
            .send(Message::Update {
                target,
                image,
                completion,
            })
            .await
            .expect("OTA server is running");
        result
    }

    fn update_via_api(
        sender: tokio::sync::mpsc::Sender<Message>,
        image: Image,
    ) -> tokio::task::JoinHandle<Result<(), Error>> {
        tokio::spawn(async move { sender.update(test_destination(), image).await })
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
        let aps_header = zb_aps::data::Header::new(
            zb_aps::Destination::Unicast(ENDPOINT),
            Cluster::OtaUpgrade.as_u16(),
            profile.as_u16(),
            ENDPOINT,
            0,
            None,
        );
        let zcl_header = Header::new(
            Scope::ClusterSpecific,
            Direction::ClientToServer,
            false,
            None,
            sequence_number,
            T::ID,
        );
        let frame = Data::raw(aps_header, Bytes::new())
            .map_payload(|_| Frame::new(zcl_header, command.into()));
        Message::Received {
            source: Source::new(test_destination().device().as_u16(), None),
            frame,
        }
    }

    async fn receive_zcl(receiver: &mut tokio::sync::mpsc::Receiver<zcl::Message>) -> ObservedZcl {
        let message = timeout(TEST_TIMEOUT, receiver.recv())
            .await
            .expect("OTA server response timed out")
            .expect("ZCL actor channel is open");
        match message {
            zcl::Message::Transmit {
                destination,
                payload,
                response,
            } => {
                complete_transmission(response);
                ObservedZcl::Transmit {
                    destination,
                    payload,
                }
            }
            zcl::Message::Reply {
                destination,
                sequence_number,
                payload,
                response,
            } => {
                complete_transmission(response);
                ObservedZcl::Reply {
                    destination,
                    sequence_number,
                    payload,
                }
            }
            other => panic!("unexpected ZCL message: {other:?}"),
        }
    }

    async fn fail_next_transmission(receiver: &mut tokio::sync::mpsc::Receiver<zcl::Message>) {
        let message = timeout(TEST_TIMEOUT, receiver.recv())
            .await
            .expect("OTA server response timed out")
            .expect("ZCL actor channel is open");
        let zcl::Message::Transmit { response, .. } = message else {
            panic!("expected OTA transmission");
        };
        let hw_response = HwResponse::new(async { Err(zb_hw::Error::NotImplemented) });
        assert!(response.send(Ok(hw_response)).is_ok());
    }

    async fn hold_next_transmission(receiver: &mut tokio::sync::mpsc::Receiver<zcl::Message>) {
        let message = timeout(TEST_TIMEOUT, receiver.recv())
            .await
            .expect("OTA server response timed out")
            .expect("ZCL actor channel is open");
        let zcl::Message::Transmit { response, .. } = message else {
            panic!("expected OTA transmission");
        };
        let hw_response = HwResponse::new(std::future::pending::<Result<(), zb_hw::Error>>());
        assert!(response.send(Ok(hw_response)).is_ok());
    }

    fn reply_bytes(message: ObservedZcl) -> (u8, Bytes) {
        let (sequence_number, metadata, bytes) = reply_parts(message);
        assert_eq!(
            metadata.tx_options(),
            zb_aps::TxOptions::ACKNOWLEDGED_TRANSMISSION
        );
        (sequence_number, bytes)
    }

    fn reply_parts(message: ObservedZcl) -> (u8, zb_hw::Metadata, Bytes) {
        let ObservedZcl::Reply {
            destination,
            sequence_number,
            payload,
        } = message
        else {
            panic!("expected OTA reply");
        };
        assert_eq!(destination, test_destination());
        let (metadata, _, bytes) = payload.into_parts();
        (sequence_number, metadata, bytes)
    }

    fn complete_transmission(
        response: tokio::sync::oneshot::Sender<Result<HwResponse, zb_hw::Error>>,
    ) {
        let hw_response = HwResponse::new(async { Ok::<(), zb_hw::Error>(()) });
        assert!(response.send(Ok(hw_response)).is_ok());
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
