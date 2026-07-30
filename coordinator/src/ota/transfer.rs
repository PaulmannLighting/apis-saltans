use std::collections::HashMap;
use std::future::{Future, poll_fn};
use std::task::Poll;
use std::time::Duration;

use le_stream::ToLeStream;
use log::{debug, trace, warn};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;
use tokio::task::{Id, JoinError, JoinSet};
use zb_aps::apsde::IndividualEndpoint;
use zb_core::destination::Device;
use zb_core::{Cluster, Direction, FullAddress, IeeeAddress};
use zb_zcl::global::default_response::DefaultResponse;
use zb_zcl::ota_upgrade::{
    Command as OtaCommand, ImageBlock, ImageBlockRequest, ImageBlockResponse,
    ImageBlockResponsePayload, ImageId, ImageNotify, ImageNotifyPayload, ImagePageRequest,
    QueryJitter, QueryNextImageRequest, QueryNextImageResponse, QueryResponse,
    QuerySpecificFileRequest, QuerySpecificFileResponse, UpgradeEndRequest, UpgradeEndResponse,
    UpgradeEndStatus,
};
use zb_zcl::{Command, Scope, Status, UnsequencedFrame, UnsequencedHeader};

use super::image::ImageTransfer;
use super::page_transfer::PageTransfer;
use super::state::RequestContext;
use super::{
    CURRENT_TIME_IMMEDIATE, Image, OTA_PROFILE, Request, UPGRADE_TIME_IMMEDIATE, UpdateError,
    UpdateResult, reply_zcl, request, request_from_unsequenced_frame, send_zcl, zcl,
};

const INITIAL_GENERATION: u64 = 0;
const GENERATION_STEP: u64 = 1;

/// Command routed from the OTA server to one destination transfer task.
pub(super) enum TransferMessage {
    /// Replace the image offered by the existing destination task.
    Replace(Box<Replacement>),
    /// Stop the update because the hardware event source is unavailable.
    HardwareUnavailable,
    /// Process an OTA request received from the destination.
    Request {
        context: RequestContext,
        command: OtaCommand,
    },
}

/// Replacement update routed to an existing destination transfer.
pub(super) struct Replacement {
    /// Complete address of the replacement update target.
    pub(super) target: FullAddress,
    /// Remote OTA client endpoint.
    pub(super) target_endpoint: IndividualEndpoint,
    /// Local OTA server endpoint used as the APS source.
    pub(super) source_endpoint: IndividualEndpoint,
    /// Replacement image.
    pub(super) image: Image,
    /// Reports the replacement update's terminal result.
    pub(super) completion: oneshot::Sender<UpdateResult>,
}

/// Normal completion notification from a destination transfer task.
pub(super) struct TransferExit {
    pub(super) destination: Device,
    pub(super) completion: oneshot::Sender<UpdateResult>,
    pub(super) result: UpdateResult,
}

/// One long-lived OTA update task for a single destination endpoint.
pub(super) struct Transfer {
    zcl: Sender<zcl::Message>,
    target: FullAddress,
    target_endpoint: IndividualEndpoint,
    source_endpoint: IndividualEndpoint,
    image: ImageTransfer,
    completion: Option<oneshot::Sender<UpdateResult>>,
    messages: Receiver<TransferMessage>,
    operations: JoinSet<OperationResult>,
    operation_generations: HashMap<Id, u64>,
    generation: u64,
}

enum OperationOutcome {
    Continue,
    Complete(UpdateResult),
}

struct OperationResult {
    generation: u64,
    outcome: OperationOutcome,
}

enum TransferEvent {
    Message(Option<TransferMessage>),
    Operation(Option<Result<(Id, OperationResult), JoinError>>),
}

impl Transfer {
    /// Create a destination transfer around its initial image and command mailbox.
    pub(super) fn new(
        zcl: Sender<zcl::Message>,
        target: FullAddress,
        target_endpoint: IndividualEndpoint,
        source_endpoint: IndividualEndpoint,
        image: Image,
        completion: oneshot::Sender<UpdateResult>,
        messages: Receiver<TransferMessage>,
    ) -> Self {
        Self {
            zcl,
            target,
            target_endpoint,
            source_endpoint,
            image: image.into_transfer(),
            completion: Some(completion),
            messages,
            operations: JoinSet::new(),
            operation_generations: HashMap::new(),
            generation: INITIAL_GENERATION,
        }
    }

    /// Run the destination transfer until it reaches a terminal outcome.
    pub(super) async fn run(mut self) -> TransferExit {
        self.notify();
        let result = loop {
            let event = poll_fn(|context| {
                if let Poll::Ready(message) = self.messages.poll_recv(context) {
                    return Poll::Ready(TransferEvent::Message(message));
                }
                if !self.operations.is_empty()
                    && let Poll::Ready(operation) = self.operations.poll_join_next_with_id(context)
                {
                    return Poll::Ready(TransferEvent::Operation(operation));
                }
                Poll::Pending
            })
            .await;
            match event {
                TransferEvent::Message(message) => {
                    let Some(message) = message else {
                        break Err(UpdateError::TransferTask);
                    };
                    if matches!(&message, TransferMessage::HardwareUnavailable) {
                        break Err(UpdateError::HardwareEventStreamClosed);
                    }
                    self.handle_message(message).await;
                }
                TransferEvent::Operation(operation) => match operation {
                    Some(Ok((task_id, operation))) => {
                        self.operation_generations.remove(&task_id);
                        if operation.generation != self.generation {
                            continue;
                        }
                        match operation.outcome {
                            OperationOutcome::Continue => {}
                            OperationOutcome::Complete(result) => break result,
                        }
                    }
                    Some(Err(error)) if error.is_cancelled() => {
                        self.operation_generations.remove(&error.id());
                    }
                    Some(Err(error)) => {
                        let operation_generation = self.operation_generations.remove(&error.id());
                        if operation_generation != Some(self.generation) {
                            continue;
                        }
                        warn!("OTA transfer operation failed: {error}");
                        break Err(UpdateError::TransferTask);
                    }
                    None => {}
                },
            }
        };
        self.operations.abort_all();
        TransferExit {
            destination: self.destination(),
            completion: self
                .completion
                .take()
                .expect("an active destination transfer retains its completion sender"),
            result,
        }
    }

    /// Apply an update replacement or dispatch an inbound OTA command.
    async fn handle_message(&mut self, message: TransferMessage) {
        match message {
            TransferMessage::Replace(replacement) => {
                let Replacement {
                    target,
                    target_endpoint,
                    source_endpoint,
                    image,
                    completion,
                } = *replacement;
                self.replace(target, target_endpoint, source_endpoint, image, completion);
            }
            TransferMessage::HardwareUnavailable => {
                unreachable!("hardware shutdown is handled by the transfer loop");
            }
            TransferMessage::Request { context, command } => {
                trace!(
                    "Processing OTA command from {}: {command:?}",
                    context.destination
                );
                match command {
                    OtaCommand::QueryNextImageRequest(request) => {
                        self.query_next_image(context, &request);
                    }
                    OtaCommand::ImageBlockRequest(request) => {
                        self.image_block(context, &request).await;
                    }
                    OtaCommand::ImagePageRequest(request) => {
                        self.image_page(context, &request).await;
                    }
                    OtaCommand::UpgradeEndRequest(request) => {
                        self.upgrade_end(context, *request);
                    }
                    OtaCommand::QuerySpecificFileRequest(request) => {
                        self.query_specific_file(context, *request);
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
        }
    }

    /// Replace the current image without replacing the destination task.
    fn replace(
        &mut self,
        target: FullAddress,
        target_endpoint: IndividualEndpoint,
        source_endpoint: IndividualEndpoint,
        image: Image,
        completion: oneshot::Sender<UpdateResult>,
    ) {
        self.operations.abort_all();
        self.generation = self.generation.wrapping_add(GENERATION_STEP);
        if let Some(previous) = self.completion.replace(completion) {
            let _result = previous.send(Err(UpdateError::Superseded));
        }
        self.target = target;
        self.target_endpoint = target_endpoint;
        self.source_endpoint = source_endpoint;
        self.image = image.into_transfer();
        self.notify();
    }

    /// Announce the currently offered image and track its hardware response.
    fn notify(&mut self) {
        let image_id = self.image.id();
        let destination = self.destination();
        let zcl = self.zcl.clone();
        trace!("Offering OTA image {image_id:?} to {destination}");
        let query_jitter =
            QueryJitter::new(QueryJitter::MAX).expect("the declared maximum query jitter is valid");
        let notification = ImageNotify::new(ImageNotifyPayload::FileVersion {
            query_jitter,
            image: image_id,
        });
        let frame = UnsequencedFrame::from_command(notification);
        let request = request_from_unsequenced_frame(
            destination.into(),
            self.source_endpoint,
            OTA_PROFILE,
            Cluster::OtaUpgrade.as_u16(),
            frame,
        );
        self.spawn_operation(async move {
            let result = transmit_command(&zcl, request).await;
            operation_outcome(result, None)
        });
    }

    /// Answer a device's discovery query with its compatible scheduled image.
    fn query_next_image(&mut self, context: RequestContext, request: &QueryNextImageRequest) {
        let offered = self.image.id();
        let current = request.image();
        let response = if self.image.upgrade_file_destination().is_some()
            || offered.manufacturer_code() != current.manufacturer_code()
            || offered.image_type() != current.image_type()
            || offered.file_version() == current.file_version()
            || !self.image.supports_hardware(request.hardware_version())
        {
            QueryResponse::NoImageAvailable
        } else {
            query_success(&self.image)
        };
        self.spawn_reply(context, QueryNextImageResponse::new(response), None);
    }

    /// Answer a destination-restricted query after validating its metadata.
    fn query_specific_file(&mut self, context: RequestContext, request: QuerySpecificFileRequest) {
        let request_address = request.request_node_address();
        let authorized = self.target.ieee_address() == request_address
            && self.image.upgrade_file_destination() == Some(request_address);
        let response = if !authorized {
            QueryResponse::NotAuthorized
        } else if self.image.id() != request.image()
            || self.image.zigbee_stack_version() != request.zigbee_stack_version()
        {
            QueryResponse::NoImageAvailable
        } else {
            query_success(&self.image)
        };
        self.spawn_reply(context, QuerySpecificFileResponse::new(response), None);
    }

    /// Read and return one requested image block.
    async fn image_block(&mut self, context: RequestContext, request: &ImageBlockRequest) {
        let request_command_id = <ImageBlockRequest as Command>::ID;
        let data = match requested_data(
            self.target,
            &self.image,
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
                let completion =
                    (status == Status::Failure).then_some(Err(UpdateError::ImageTransfer));
                self.spawn_default_response(context, request_command_id, status, completion);
                return;
            }
        };

        let block = ImageBlock::try_new(request.image(), request.file_offset(), data)
            .expect("requested OTA blocks never exceed the client's u8 maximum data size");
        let response = ImageBlockResponse::new(ImageBlockResponsePayload::Success(block));
        self.spawn_reply(context, response, None);
    }

    /// Start a paced image-page operation owned by this destination task.
    async fn image_page(&mut self, context: RequestContext, request: &ImagePageRequest) {
        let request_command_id = <ImagePageRequest as Command>::ID;
        if request.page_size() == 0 {
            self.spawn_default_response(
                context,
                request_command_id,
                Status::MalformedCommand,
                None,
            );
            return;
        }
        let first_block = match requested_data(
            self.target,
            &self.image,
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
                let completion =
                    (status == Status::Failure).then_some(Err(UpdateError::ImageTransfer));
                self.spawn_default_response(context, request_command_id, status, completion);
                return;
            }
        };

        let image = self.image.clone();
        let image_id = request.image();
        let maximum_data_size = usize::from(request.maximum_data_size());
        let page_end = usize::try_from(request.file_offset())
            .unwrap_or(usize::MAX)
            .saturating_add(usize::from(request.page_size()))
            .min(image.len());
        let spacing = Duration::from_millis(u64::from(request.response_spacing()));
        let operation = PageTransfer {
            zcl: self.zcl.clone(),
            image,
            destination: context.destination,
            source_endpoint: context.source_endpoint,
            image_id,
            maximum_data_size,
            page_end,
            spacing,
            offset: usize::try_from(request.file_offset())
                .expect("validated OTA file offset fits usize"),
            sequence_number: context.sequence_number,
            block_data: first_block,
        };
        self.spawn_operation(async move { operation_outcome(operation.run().await, None) });
    }

    /// Complete or acknowledge an upgrade attempt according to the client status.
    fn upgrade_end(&mut self, context: RequestContext, request: UpgradeEndRequest) {
        let request_command_id = <UpgradeEndRequest as Command>::ID;
        if self.image.id() != request.image() {
            self.spawn_default_response(
                context,
                request_command_id,
                Status::NoImageAvailable,
                None,
            );
            return;
        }

        match request.status() {
            UpgradeEndStatus::Success => {
                let response = UpgradeEndResponse::new(
                    request.image(),
                    CURRENT_TIME_IMMEDIATE,
                    UPGRADE_TIME_IMMEDIATE,
                );
                self.spawn_reply(context, response, Some(Ok(())));
            }
            status @ (UpgradeEndStatus::Abort
            | UpgradeEndStatus::InvalidImage
            | UpgradeEndStatus::RequireMoreImage) => {
                let error = match status {
                    UpgradeEndStatus::Abort => UpdateError::Aborted,
                    UpgradeEndStatus::InvalidImage => UpdateError::InvalidImage,
                    UpgradeEndStatus::RequireMoreImage => UpdateError::RequireMoreImage,
                    UpgradeEndStatus::Success => unreachable!("success is handled separately"),
                };
                self.spawn_default_response(
                    context,
                    request_command_id,
                    Status::Success,
                    Some(Err(error)),
                );
            }
        }
    }

    /// Spawn one reply operation inside this destination transfer.
    fn spawn_reply<T>(
        &mut self,
        context: RequestContext,
        command: T,
        completion: Option<UpdateResult>,
    ) where
        T: Command + zb_zcl::Directed + zb_zcl::Scoped + ToLeStream,
    {
        let zcl = self.zcl.clone();
        let request = request(
            context.destination.into(),
            context.source_endpoint,
            OTA_PROFILE,
            Cluster::OtaUpgrade.as_u16(),
            command,
        );
        self.spawn_operation(async move {
            let result = transmit_reply(&zcl, context.sequence_number, request).await;
            operation_outcome(result, completion)
        });
    }

    /// Spawn a generation-tagged operation owned by this destination transfer.
    fn spawn_operation<T>(&mut self, operation: T)
    where
        T: Future<Output = OperationOutcome> + Send + 'static,
    {
        let generation = self.generation;
        let task = self.operations.spawn(async move {
            OperationResult {
                generation,
                outcome: operation.await,
            }
        });
        self.operation_generations.insert(task.id(), generation);
    }

    /// Spawn a global default-response operation inside this destination transfer.
    fn spawn_default_response(
        &mut self,
        context: RequestContext,
        request_command_id: u8,
        status: Status,
        completion: Option<UpdateResult>,
    ) {
        let response = DefaultResponse::new(request_command_id, status.into());
        let frame = UnsequencedFrame::new(
            UnsequencedHeader::new(
                Scope::Global,
                Direction::ServerToClient,
                true,
                None,
                <DefaultResponse as Command>::ID,
            ),
            response.to_le_stream().collect(),
        );
        let request = request_from_unsequenced_frame(
            context.destination.into(),
            context.source_endpoint,
            OTA_PROFILE,
            Cluster::OtaUpgrade.as_u16(),
            frame,
        );
        let zcl = self.zcl.clone();
        self.spawn_operation(async move {
            let result = transmit_reply(&zcl, context.sequence_number, request).await;
            operation_outcome(result, completion)
        });
    }

    const fn destination(&self) -> Device {
        Device::new(self.target.short_id(), self.target_endpoint.get())
    }
}

const fn query_success(image: &ImageTransfer) -> QueryResponse {
    QueryResponse::Success {
        image: image.id(),
        image_size: image.image_size(),
    }
}

fn operation_outcome(result: UpdateResult, completion: Option<UpdateResult>) -> OperationOutcome {
    match result {
        Ok(()) => completion.map_or(OperationOutcome::Continue, OperationOutcome::Complete),
        Err(error) => OperationOutcome::Complete(Err(error)),
    }
}

async fn transmit_command(zcl: &Sender<zcl::Message>, request: Request) -> UpdateResult {
    let Some(()) = send_zcl(zcl, request).await else {
        return Err(UpdateError::Transmission);
    };
    Ok(())
}

async fn transmit_reply(
    zcl: &Sender<zcl::Message>,
    sequence_number: u8,
    request: Request,
) -> UpdateResult {
    let Some(()) = reply_zcl(zcl, sequence_number, request).await else {
        return Err(UpdateError::Transmission);
    };
    Ok(())
}

async fn requested_data(
    target: FullAddress,
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
    if !request_address_is_authorized(target, image, request_node_address) {
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

pub(super) async fn read_image_range(
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
    target: FullAddress,
    image: &ImageTransfer,
    request_node_address: Option<IeeeAddress>,
) -> bool {
    if let Some(request_address) = request_node_address
        && target.ieee_address() != request_address
    {
        return false;
    }

    image
        .upgrade_file_destination()
        .is_none_or(|destination| target.ieee_address() == destination)
}
