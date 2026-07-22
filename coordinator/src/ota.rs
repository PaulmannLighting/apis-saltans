//! Coordinator-owned OTA Upgrade server.

use std::collections::BTreeMap;
use std::future::poll_fn;
use std::task::Poll;
use std::time::Duration;

use le_stream::ToLeStream;
use log::{debug, trace, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender, UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot;
use tokio::task::JoinSet;
use zb_aps::Data;
use zb_core::destination::Device;
use zb_core::{Cluster, Direction, IeeeAddress, Profile};
use zb_hw::HwResponse;
use zb_nwk::Source;
use zb_zcl::global::default_response::DefaultResponse;
use zb_zcl::ota_upgrade::{
    Command as OtaCommand, ImageBlock, ImageBlockRequest, ImageBlockResponse,
    ImageBlockResponsePayload, ImageId, ImageNotify, ImageNotifyPayload, ImagePageRequest,
    QueryJitter, QueryNextImageRequest, QueryNextImageResponse, QueryResponse,
    QuerySpecificFileRequest, QuerySpecificFileResponse, UpgradeEndRequest, UpgradeEndResponse,
    UpgradeEndStatus,
};
use zb_zcl::{Command, Frame, Scope, Status};

use self::image::ImageTransfer;
pub use self::image::{
    BaseHeaderBytes, FieldControl, Header, HeaderString, Image, ParseImage, ParseImageError,
};
pub use self::message::{Message, UpdateError, UpdateResult};
use self::page_transfer::PageTransfer;
use self::state::{RequestContext, ScheduledUpdate};
use self::transfer::{ServerEvent, TransferEvent, TransferKey, report_failure};
use crate::zcl::{self, Metadata, Payload};

mod image;
mod message;
mod page_transfer;
mod state;
mod transfer;

const CURRENT_TIME_IMMEDIATE: u32 = 0;
const UPGRADE_TIME_IMMEDIATE: u32 = 0;
const INITIAL_UPDATE_GENERATION: u64 = 0;
const UPDATE_GENERATION_STEP: u64 = 1;
const OTA_PROFILE: Profile = Profile::ZigbeeHomeAutomation;

/// Stateful OTA Upgrade server actor.
#[derive(Debug)]
pub struct Server {
    zcl: Sender<zcl::Message>,
    inbound: Receiver<Message>,
    updates: BTreeMap<Device, ScheduledUpdate>,
    transmissions: JoinSet<()>,
    transfer_events: UnboundedReceiver<TransferEvent>,
    transfer_events_out: UnboundedSender<TransferEvent>,
    next_update_generation: u64,
}

impl Server {
    /// Create an empty OTA server attached to its ZCL sender and inbound command channel.
    fn new(zcl: Sender<zcl::Message>, inbound: Receiver<Message>) -> Self {
        let (transfer_events_out, transfer_events) = tokio::sync::mpsc::unbounded_channel();
        Self {
            zcl,
            inbound,
            updates: BTreeMap::new(),
            transmissions: JoinSet::new(),
            transfer_events,
            transfer_events_out,
            next_update_generation: INITIAL_UPDATE_GENERATION,
        }
    }

    /// Process scheduled updates and inbound OTA commands until the inbound channel closes.
    pub async fn run(mut self) {
        loop {
            self.reap_transmissions();
            let event = poll_fn(|context| {
                if let Poll::Ready(Some(event)) = self.transfer_events.poll_recv(context) {
                    Poll::Ready(ServerEvent::Transfer(event))
                } else if let Poll::Ready(message) = self.inbound.poll_recv(context) {
                    Poll::Ready(ServerEvent::Message(message))
                } else {
                    Poll::Pending
                }
            })
            .await;
            match event {
                ServerEvent::Message(Some(Message::Update {
                    target,
                    image,
                    completion,
                })) => self.update(target, image, completion).await,
                ServerEvent::Message(Some(Message::Received { source, frame })) => {
                    self.received(source, frame).await;
                }
                ServerEvent::Message(None) => break,
                ServerEvent::Transfer(event) => {
                    self.complete_update(event.key, Err(event.error));
                }
            }
        }
        self.reap_transmissions();
    }

    /// Spawn the OTA server actor on the current Tokio runtime.
    pub(crate) fn spawn(zcl: Sender<zcl::Message>, receiver: Receiver<Message>) {
        spawn(Self::new(zcl, receiver).run());
    }

    /// Replace the update scheduled for `target` and announce the new image to that device.
    async fn update(
        &mut self,
        target: Device,
        image: Image,
        completion: oneshot::Sender<UpdateResult>,
    ) {
        let image_id = image.id();
        let transfer = image.into_transfer();
        let generation = self.next_update_generation;
        self.next_update_generation = self
            .next_update_generation
            .wrapping_add(UPDATE_GENERATION_STEP);
        let key = TransferKey {
            destination: target,
            generation,
        };
        trace!("Offering OTA image {image_id:?} to {target}");
        if let Some(previous) = self.updates.insert(
            target,
            ScheduledUpdate {
                transfer,
                completion,
                generation,
            },
        ) {
            let _result = previous.completion.send(Err(UpdateError::Superseded));
        }

        let query_jitter =
            QueryJitter::new(QueryJitter::MAX).expect("the declared maximum query jitter is valid");
        let notification = ImageNotify::new(ImageNotifyPayload::FileVersion {
            query_jitter,
            image: image_id,
        });
        let payload = Payload::from(notification)
            .with_profile(OTA_PROFILE)
            .with_disable_default_response(false);
        let response = send_zcl(&self.zcl, target.into(), payload).await;
        if response.is_none() {
            self.complete_update(key, Err(UpdateError::Transmission));
        } else {
            self.track_transmission(response, Some(key));
        }
    }

    /// Validate the source metadata and dispatch an inbound frame to its command handler.
    async fn received(&mut self, source: Source, frame: Data<Frame<OtaCommand>>) {
        let aps_header = frame.header();
        let Ok(endpoint) = aps_header.source_endpoint().inspect_err(|error| {
            warn!("Discarding OTA command with invalid source endpoint: {error:?}");
        }) else {
            return;
        };
        let Ok(profile) = aps_header.profile().inspect_err(|profile_id| {
            warn!("Discarding OTA command with unknown profile {profile_id:#06x}");
        }) else {
            return;
        };
        if profile != OTA_PROFILE {
            warn!("Discarding OTA command with unsupported profile {profile}");
            return;
        }
        let Ok(short_id) = source.node_id().try_into().inspect_err(|node_id| {
            warn!("Discarding OTA command from invalid node ID {node_id:#06x}");
        }) else {
            return;
        };

        let (_, zcl_frame) = frame.into_parts();
        let (zcl_header, command) = zcl_frame.into_parts();
        let context = RequestContext {
            destination: Device::new(short_id, endpoint),
            source_ieee_address: source.ieee_address(),
            sequence_number: zcl_header.seq(),
        };

        trace!(
            "Processing OTA command from {}: {command:?}",
            context.destination
        );
        match command {
            OtaCommand::QueryNextImageRequest(request) => {
                self.query_next_image(context, &request).await;
            }
            OtaCommand::ImageBlockRequest(request) => {
                self.image_block(context, &request).await;
            }
            OtaCommand::ImagePageRequest(request) => {
                self.image_page(context, &request).await;
            }
            OtaCommand::UpgradeEndRequest(request) => {
                self.upgrade_end(context, *request).await;
            }
            OtaCommand::QuerySpecificFileRequest(request) => {
                self.query_specific_file(context, *request).await;
            }
            OtaCommand::ImageNotify(_)
            | OtaCommand::QueryNextImageResponse(_)
            | OtaCommand::ImageBlockResponse(_)
            | OtaCommand::UpgradeEndResponse(_)
            | OtaCommand::QuerySpecificFileResponse(_) => {
                debug!(
                    "Ignoring server-to-client OTA command from {}",
                    context.destination
                );
            }
        }
    }

    /// Answer a device's discovery query with its compatible scheduled image, if any.
    async fn query_next_image(&mut self, context: RequestContext, request: &QueryNextImageRequest) {
        let response = self
            .update_for(context)
            .map_or(QueryResponse::NotAuthorized, |update| {
                let offered = update.transfer.id();
                let current = request.image();
                if update.transfer.upgrade_file_destination().is_some()
                    || offered.manufacturer_code() != current.manufacturer_code()
                    || offered.image_type() != current.image_type()
                    || offered.file_version() == current.file_version()
                    || !update
                        .transfer
                        .supports_hardware(request.hardware_version())
                {
                    QueryResponse::NoImageAvailable
                } else {
                    query_success(&update.transfer)
                }
            });
        self.reply(context, QueryNextImageResponse::new(response).into())
            .await;
    }

    /// Answer a destination-restricted query after validating its IEEE address and image metadata.
    async fn query_specific_file(
        &mut self,
        context: RequestContext,
        request: QuerySpecificFileRequest,
    ) {
        let response = self
            .update_for(context)
            .map_or(QueryResponse::NotAuthorized, |update| {
                let request_address = request.request_node_address();
                let authorized = context.source_ieee_address == Some(request_address)
                    && update.transfer.upgrade_file_destination() == Some(request_address);
                if !authorized {
                    QueryResponse::NotAuthorized
                } else if update.transfer.id() != request.image()
                    || update.transfer.zigbee_stack_version() != request.zigbee_stack_version()
                {
                    QueryResponse::NoImageAvailable
                } else {
                    query_success(&update.transfer)
                }
            });
        self.reply(context, QuerySpecificFileResponse::new(response).into())
            .await;
    }

    /// Read and return one requested image block, or emit the corresponding default response.
    async fn image_block(&mut self, context: RequestContext, request: &ImageBlockRequest) {
        let request_command_id = <ImageBlockRequest as Command>::ID;
        let Some(update) = self.update_for(context) else {
            self.default_response(context, request_command_id, Status::NotAuthorized)
                .await;
            return;
        };
        let data = match requested_data(
            context,
            &update.transfer,
            request.image(),
            request.file_offset(),
            request.maximum_data_size(),
            request.request_node_address(),
            None,
        )
        .await
        {
            Ok(data) => data,
            Err(status) => {
                if status == Status::Failure {
                    self.complete_update(
                        TransferKey {
                            destination: context.destination,
                            generation: update.generation,
                        },
                        Err(UpdateError::ImageTransfer),
                    );
                }
                self.default_response(context, request_command_id, status)
                    .await;
                return;
            }
        };

        let block = ImageBlock::try_new(request.image(), request.file_offset(), data)
            .expect("requested OTA blocks never exceed the client's u8 maximum data size");
        let response = ImageBlockResponse::new(ImageBlockResponsePayload::Success(block));
        self.reply(context, response.into()).await;
    }

    /// Start a paced background transfer for the blocks covered by an image page request.
    ///
    /// The first block is validated and read before spawning the transfer. The background task
    /// advances transaction sequence numbers, disables APS acknowledgements, and stops on the
    /// first read or transmission failure.
    async fn image_page(&mut self, context: RequestContext, request: &ImagePageRequest) {
        let request_command_id = <ImagePageRequest as Command>::ID;
        let Some(update) = self.update_for(context) else {
            self.default_response(context, request_command_id, Status::NotAuthorized)
                .await;
            return;
        };
        if request.page_size() == 0 {
            self.default_response(context, request_command_id, Status::MalformedCommand)
                .await;
            return;
        }
        let first_block = match requested_data(
            context,
            &update.transfer,
            request.image(),
            request.file_offset(),
            request.maximum_data_size(),
            request.request_node_address(),
            Some(request.page_size()),
        )
        .await
        {
            Ok(data) => data,
            Err(status) => {
                if status == Status::Failure {
                    self.complete_update(
                        TransferKey {
                            destination: context.destination,
                            generation: update.generation,
                        },
                        Err(UpdateError::ImageTransfer),
                    );
                }
                self.default_response(context, request_command_id, status)
                    .await;
                return;
            }
        };

        let transfer = update.transfer.clone();
        let image_id = request.image();
        let maximum_data_size = usize::from(request.maximum_data_size());
        let page_end = usize::try_from(request.file_offset())
            .unwrap_or(usize::MAX)
            .saturating_add(usize::from(request.page_size()))
            .min(transfer.len());
        let spacing = Duration::from_millis(u64::from(request.response_spacing()));
        self.transmissions.spawn(
            PageTransfer {
                zcl: self.zcl.clone(),
                image: transfer,
                events: self.transfer_events_out.clone(),
                key: TransferKey {
                    destination: context.destination,
                    generation: update.generation,
                },
                destination: context.destination,
                image_id,
                maximum_data_size,
                page_end,
                spacing,
                offset: usize::try_from(request.file_offset())
                    .expect("validated OTA file offset fits usize"),
                sequence_number: context.sequence_number,
                block_data: first_block,
            }
            .run(),
        );
    }

    /// Complete or acknowledge an upgrade attempt according to the client's reported status.
    async fn upgrade_end(&mut self, context: RequestContext, request: UpgradeEndRequest) {
        let request_command_id = <UpgradeEndRequest as Command>::ID;
        let Some(update) = self.update_for(context) else {
            self.default_response(context, request_command_id, Status::NotAuthorized)
                .await;
            return;
        };
        if update.transfer.id() != request.image() {
            self.default_response(context, request_command_id, Status::NoImageAvailable)
                .await;
            return;
        }
        let generation = update.generation;

        match request.status() {
            UpgradeEndStatus::Success => {
                let response = UpgradeEndResponse::new(
                    request.image(),
                    CURRENT_TIME_IMMEDIATE,
                    UPGRADE_TIME_IMMEDIATE,
                );
                self.reply(context, response.into()).await;
                self.complete_update(
                    TransferKey {
                        destination: context.destination,
                        generation,
                    },
                    Ok(()),
                );
            }
            status @ (UpgradeEndStatus::Abort
            | UpgradeEndStatus::InvalidImage
            | UpgradeEndStatus::RequireMoreImage) => {
                self.default_response(context, request_command_id, Status::Success)
                    .await;
                let error = match status {
                    UpgradeEndStatus::Abort => UpdateError::Aborted,
                    UpgradeEndStatus::InvalidImage => UpdateError::InvalidImage,
                    UpgradeEndStatus::RequireMoreImage => UpdateError::RequireMoreImage,
                    UpgradeEndStatus::Success => unreachable!("success is handled separately"),
                };
                self.complete_update(
                    TransferKey {
                        destination: context.destination,
                        generation,
                    },
                    Err(error),
                );
            }
        }
    }

    /// Find the update authorized for the request's device endpoint.
    fn update_for(&self, context: RequestContext) -> Option<&ScheduledUpdate> {
        self.updates.get(&context.destination)
    }

    /// Send a cluster-specific reply with the request sequence number and track its completion.
    async fn reply(&mut self, context: RequestContext, payload: Payload) {
        let key = self.update_for(context).map(|update| TransferKey {
            destination: context.destination,
            generation: update.generation,
        });
        let response = reply_zcl(
            &self.zcl,
            context.destination,
            OTA_PROFILE,
            context.sequence_number,
            payload,
        )
        .await;
        if response.is_none() {
            if let Some(key) = key {
                self.complete_update(key, Err(UpdateError::Transmission));
            }
        } else {
            self.track_transmission(response, key);
        }
    }

    /// Send a global default response for a rejected or acknowledged client command.
    async fn default_response(
        &mut self,
        context: RequestContext,
        request_command_id: u8,
        status: Status,
    ) {
        let response = DefaultResponse::new(request_command_id, status.into());
        let payload = Payload::new(
            zb_hw::Metadata::new(OTA_PROFILE, Cluster::OtaUpgrade.as_u16()),
            Metadata::new(
                Scope::Global,
                Direction::ServerToClient,
                true,
                None,
                <DefaultResponse as Command>::ID,
            ),
            response.to_le_stream().collect(),
        );
        self.reply(context, payload).await;
    }

    /// Poll a deferred hardware response in a tracked task without blocking the server actor.
    fn track_transmission(&mut self, response: Option<HwResponse>, key: Option<TransferKey>) {
        if let Some(response) = response {
            let transfer_events = self.transfer_events_out.clone();
            self.transmissions.spawn(async move {
                if let Err(error) = response.await {
                    warn!("OTA transmission failed: {error}");
                    if let Some(key) = key {
                        report_failure(&transfer_events, key, UpdateError::Transmission);
                    }
                }
            });
        }
    }

    /// Resolve and remove a scheduled update when `key` still identifies its current image.
    fn complete_update(&mut self, key: TransferKey, result: UpdateResult) {
        let is_current = self
            .updates
            .get(&key.destination)
            .is_some_and(|update| update.generation == key.generation);
        if is_current {
            let update = self
                .updates
                .remove(&key.destination)
                .expect("the current OTA update remains present");
            let _result = update.completion.send(result);
        }
    }

    /// Remove completed transmission tasks and report task failures.
    fn reap_transmissions(&mut self) {
        while let Some(result) = self.transmissions.try_join_next() {
            if let Err(error) = result {
                warn!("OTA transmission task failed: {error}");
            }
        }
    }
}

const fn query_success(image: &ImageTransfer) -> QueryResponse {
    QueryResponse::Success {
        image: image.id(),
        image_size: image.image_size(),
    }
}

async fn requested_data(
    context: RequestContext,
    image: &ImageTransfer,
    requested_image: ImageId,
    file_offset: u32,
    maximum_data_size: u8,
    request_node_address: Option<IeeeAddress>,
    page_size: Option<u16>,
) -> Result<Box<[u8]>, Status> {
    if image.id() != requested_image {
        return Err(Status::NoImageAvailable);
    }
    if maximum_data_size == 0 {
        return Err(Status::MalformedCommand);
    }
    if !request_address_is_authorized(context, image, request_node_address) {
        return Err(Status::NotAuthorized);
    }

    let offset = usize::try_from(file_offset).map_err(|_| Status::MalformedCommand)?;
    if offset >= image.len() {
        return Err(Status::MalformedCommand);
    }
    let mut length = usize::from(maximum_data_size);
    if let Some(page_size) = page_size {
        length = length.min(usize::from(page_size));
    }
    let end = offset.saturating_add(length).min(image.len());
    read_image_range(image, offset, end - offset).await
}

async fn read_image_range(
    image: &ImageTransfer,
    offset: usize,
    length: usize,
) -> Result<Box<[u8]>, Status> {
    image.read_range(offset, length).await.map_err(|error| {
        warn!("Failed to read OTA image data: {error}");
        Status::Failure
    })
}

fn request_address_is_authorized(
    context: RequestContext,
    image: &ImageTransfer,
    request_node_address: Option<IeeeAddress>,
) -> bool {
    if let Some(request_address) = request_node_address
        && context.source_ieee_address != Some(request_address)
    {
        return false;
    }

    image.upgrade_file_destination().is_none_or(|destination| {
        context.source_ieee_address == Some(destination)
            && request_node_address == Some(destination)
    })
}

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
    fn scheduling_update_sends_unicast_image_notify() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, ota_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            tokio::spawn(Server::new(zcl_sender, ota_receiver).run());
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
    fn ignores_requests_outside_the_home_automation_profile() {
        run_test(async {
            let (zcl_sender, mut zcl_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            let (ota_sender, ota_receiver) = tokio::sync::mpsc::channel(TEST_CHANNEL_SIZE);
            tokio::spawn(Server::new(zcl_sender, ota_receiver).run());
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
            tokio::spawn(Server::new(zcl_sender, ota_receiver).run());
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
            tokio::spawn(Server::new(zcl_sender, ota_receiver).run());
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
            tokio::spawn(Server::new(zcl_sender, ota_receiver).run());
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
            tokio::spawn(Server::new(zcl_sender, ota_receiver).run());
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
                assert!(!metadata.aps_acknowledgement());
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

    async fn schedule(
        sender: &tokio::sync::mpsc::Sender<Message>,
        image: Image,
    ) -> tokio::sync::oneshot::Receiver<UpdateResult> {
        let (completion, result) = tokio::sync::oneshot::channel();
        sender
            .send(Message::Update {
                target: test_destination(),
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

    fn reply_bytes(message: ObservedZcl) -> (u8, Bytes) {
        let (sequence_number, metadata, bytes) = reply_parts(message);
        assert!(metadata.aps_acknowledgement());
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
