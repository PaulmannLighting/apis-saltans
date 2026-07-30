use std::collections::HashMap;
use std::future::{Future, poll_fn};
use std::task::Poll;
use std::time::Duration;

use le_stream::ToLeStream;
use log::{debug, trace, warn};
use tokio::sync::mpsc::{Receiver, Sender, WeakSender};
use tokio::sync::oneshot;
use tokio::task::{AbortHandle, Id, JoinError, JoinSet};
use tokio::time::sleep;
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
    UpdateResult, UpdateTimeouts, reply_zcl, request, request_from_unsequenced_frame, send_zcl,
    zcl,
};

const INITIAL_GENERATION: u64 = 0;
const GENERATION_STEP: u64 = 1;

/// Command routed from the OTA server to one destination transfer task.
pub(super) enum TransferMessage {
    /// Replace the image offered by the existing destination task.
    Replace(Box<Offer>),
    /// Cancel an update whose caller dropped or explicitly cancelled its future.
    Cancel {
        /// Generation of the update that owned the cancellation signal.
        generation: u64,
    },
    /// Expire one lifecycle deadline.
    Deadline {
        /// Generation of the update that scheduled the deadline.
        generation: u64,
        /// Deadline that expired.
        deadline: Deadline,
    },
    /// Stop the update because the hardware event source is unavailable.
    HardwareUnavailable,
    /// Process an OTA request received from the destination.
    Request {
        context: RequestContext,
        command: OtaCommand,
    },
}

/// Complete resources and lifecycle policy for one OTA image offer.
pub(super) struct Offer {
    /// Complete address of the update target.
    pub(super) target: FullAddress,
    /// Remote OTA client endpoint.
    pub(super) target_endpoint: IndividualEndpoint,
    /// Local OTA server endpoint used as the APS source.
    pub(super) source_endpoint: IndividualEndpoint,
    /// Offered image.
    pub(super) image: Image,
    /// Deadlines selected for the offer.
    pub(super) timeouts: UpdateTimeouts,
    /// Resolves when the caller drops or cancels its update future.
    pub(super) cancellation: oneshot::Receiver<()>,
    /// Reports the update's terminal result.
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
    sender: WeakSender<TransferMessage>,
    target: FullAddress,
    target_endpoint: IndividualEndpoint,
    source_endpoint: IndividualEndpoint,
    image: ImageTransfer,
    timeouts: UpdateTimeouts,
    cancellation: Option<oneshot::Receiver<()>>,
    completion: Option<oneshot::Sender<UpdateResult>>,
    messages: Receiver<TransferMessage>,
    operations: JoinSet<OperationResult>,
    operation_generations: HashMap<Id, u64>,
    cancellation_task: Option<AbortHandle>,
    discovery_deadline_task: Option<AbortHandle>,
    inactivity_deadline_task: Option<AbortHandle>,
    total_deadline_task: Option<AbortHandle>,
    generation: u64,
}

/// Lifecycle deadline enforced for an OTA update.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Deadline {
    /// The client has not accepted the offered image.
    Discovery,
    /// The client stopped requesting image data.
    BlockInactivity,
    /// The complete exchange exceeded its maximum duration.
    TotalTransfer,
}

enum OperationOutcome {
    Continue,
    Complete(UpdateResult),
}

struct OperationResult {
    generation: u64,
    outcome: OperationOutcome,
}

struct ImageRange {
    offset: usize,
    length: usize,
}

enum TransferEvent {
    Message(Option<TransferMessage>),
    Operation(Option<Result<(Id, OperationResult), JoinError>>),
}

impl Transfer {
    /// Create a destination transfer around its initial image and command mailbox.
    pub(super) fn new(
        zcl: Sender<zcl::Message>,
        sender: WeakSender<TransferMessage>,
        offer: Offer,
        messages: Receiver<TransferMessage>,
    ) -> Self {
        let Offer {
            target,
            target_endpoint,
            source_endpoint,
            image,
            timeouts,
            cancellation,
            completion,
        } = offer;
        Self {
            zcl,
            sender,
            target,
            target_endpoint,
            source_endpoint,
            image: image.into_transfer(),
            timeouts,
            cancellation: Some(cancellation),
            completion: Some(completion),
            messages,
            operations: JoinSet::new(),
            operation_generations: HashMap::new(),
            cancellation_task: None,
            discovery_deadline_task: None,
            inactivity_deadline_task: None,
            total_deadline_task: None,
            generation: INITIAL_GENERATION,
        }
    }

    /// Run the destination transfer until it reaches a terminal outcome.
    pub(super) async fn run(mut self) -> TransferExit {
        self.start_lifecycle();
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
                    match message {
                        TransferMessage::HardwareUnavailable => {
                            break Err(UpdateError::HardwareEventStreamClosed);
                        }
                        TransferMessage::Cancel { generation } if generation == self.generation => {
                            break Err(UpdateError::Cancelled);
                        }
                        TransferMessage::Deadline {
                            generation,
                            deadline,
                        } if generation == self.generation => {
                            break Err(deadline.error());
                        }
                        message => self.handle_message(message),
                    }
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
        self.abort_lifecycle_tasks();
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
    fn handle_message(&mut self, message: TransferMessage) {
        match message {
            TransferMessage::Replace(replacement) => {
                self.replace(*replacement);
            }
            TransferMessage::Cancel { .. } | TransferMessage::Deadline { .. } => {
                // Stale lifecycle messages from a superseded generation are ignored.
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
                        if self.query_next_image(context, &request) {
                            self.record_transfer_activity();
                        }
                    }
                    OtaCommand::ImageBlockRequest(request) => {
                        if self.image_block(context, *request) {
                            self.record_transfer_activity();
                        }
                    }
                    OtaCommand::ImagePageRequest(request) => {
                        if self.image_page(context, *request) {
                            self.record_transfer_activity();
                        }
                    }
                    OtaCommand::UpgradeEndRequest(request) => {
                        self.upgrade_end(context, *request);
                    }
                    OtaCommand::QuerySpecificFileRequest(request) => {
                        if self.query_specific_file(context, *request) {
                            self.record_transfer_activity();
                        }
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
    fn replace(&mut self, offer: Offer) {
        let Offer {
            target,
            target_endpoint,
            source_endpoint,
            image,
            timeouts,
            cancellation,
            completion,
        } = offer;
        self.abort_lifecycle_tasks();
        self.operations.abort_all();
        self.generation = self.generation.wrapping_add(GENERATION_STEP);
        if let Some(previous) = self.completion.replace(completion) {
            let _result = previous.send(Err(UpdateError::Superseded));
        }
        self.target = target;
        self.target_endpoint = target_endpoint;
        self.source_endpoint = source_endpoint;
        self.image = image.into_transfer();
        self.timeouts = timeouts;
        self.cancellation = Some(cancellation);
        self.start_lifecycle();
        self.notify();
    }

    /// Start cancellation, discovery, and total-transfer lifecycle tasks for this generation.
    fn start_lifecycle(&mut self) {
        let generation = self.generation;
        let cancellation = self
            .cancellation
            .take()
            .expect("a new OTA generation retains its cancellation receiver");
        self.cancellation_task = Some(spawn_cancellation(
            self.sender.clone(),
            generation,
            cancellation,
        ));
        self.discovery_deadline_task = Some(spawn_deadline(
            self.sender.clone(),
            generation,
            Deadline::Discovery,
            self.timeouts.discovery(),
        ));
        self.total_deadline_task = Some(spawn_deadline(
            self.sender.clone(),
            generation,
            Deadline::TotalTransfer,
            self.timeouts.total_transfer(),
        ));
    }

    /// Move from discovery into transfer activity and reset the inactivity deadline.
    fn record_transfer_activity(&mut self) {
        abort_task(&mut self.discovery_deadline_task);
        abort_task(&mut self.inactivity_deadline_task);
        self.inactivity_deadline_task = Some(spawn_deadline(
            self.sender.clone(),
            self.generation,
            Deadline::BlockInactivity,
            self.timeouts.block_inactivity(),
        ));
    }

    /// Abort every lifecycle task owned by the current generation.
    fn abort_lifecycle_tasks(&mut self) {
        abort_task(&mut self.cancellation_task);
        abort_task(&mut self.discovery_deadline_task);
        abort_task(&mut self.inactivity_deadline_task);
        abort_task(&mut self.total_deadline_task);
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
    fn query_next_image(
        &mut self,
        context: RequestContext,
        request: &QueryNextImageRequest,
    ) -> bool {
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
        let accepted = matches!(response, QueryResponse::Success { .. });
        self.spawn_reply(context, QueryNextImageResponse::new(response), None);
        accepted
    }

    /// Answer a destination-restricted query after validating its metadata.
    fn query_specific_file(
        &mut self,
        context: RequestContext,
        request: QuerySpecificFileRequest,
    ) -> bool {
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
        let accepted = matches!(response, QueryResponse::Success { .. });
        self.spawn_reply(context, QuerySpecificFileResponse::new(response), None);
        accepted
    }

    /// Start a generation-owned operation that reads and returns one requested image block.
    fn image_block(&mut self, context: RequestContext, request: ImageBlockRequest) -> bool {
        let range = match requested_range(
            self.target,
            &self.image,
            request.image(),
            request.file_offset(),
            request.maximum_data_size(),
            request.request_node_address(),
            None,
        ) {
            Ok(range) => range,
            Err(status) => {
                self.spawn_default_response(
                    context,
                    <ImageBlockRequest as Command>::ID,
                    status,
                    None,
                );
                return false;
            }
        };
        let zcl = self.zcl.clone();
        let image = self.image.clone();
        self.spawn_operation(async move {
            image_block_operation(&zcl, &image, context, request, range).await
        });
        true
    }

    /// Start a paced image-page operation owned by this destination task.
    fn image_page(&mut self, context: RequestContext, request: ImagePageRequest) -> bool {
        let request_command_id = <ImagePageRequest as Command>::ID;
        if request.page_size() == 0 {
            self.spawn_default_response(
                context,
                request_command_id,
                Status::MalformedCommand,
                None,
            );
            return false;
        }

        let range = match requested_range(
            self.target,
            &self.image,
            request.image(),
            request.file_offset(),
            request.maximum_data_size(),
            request.request_node_address(),
            Some(request.page_size()),
        ) {
            Ok(range) => range,
            Err(status) => {
                self.spawn_default_response(context, request_command_id, status, None);
                return false;
            }
        };
        let zcl = self.zcl.clone();
        let image = self.image.clone();
        self.spawn_operation(async move {
            image_page_operation(zcl, image, context, request, range).await
        });
        true
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
        let request = default_response_request(context, request_command_id, status);
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

impl Drop for Transfer {
    fn drop(&mut self) {
        self.abort_lifecycle_tasks();
    }
}

impl Deadline {
    const fn error(self) -> UpdateError {
        match self {
            Self::Discovery => UpdateError::DiscoveryTimeout,
            Self::BlockInactivity => UpdateError::BlockInactivityTimeout,
            Self::TotalTransfer => UpdateError::TotalTransferTimeout,
        }
    }
}

fn spawn_cancellation(
    sender: WeakSender<TransferMessage>,
    generation: u64,
    cancellation: oneshot::Receiver<()>,
) -> AbortHandle {
    let task = tokio::spawn(async move {
        let _result = cancellation.await;
        let Some(sender) = sender.upgrade() else {
            return;
        };
        let _result = sender.send(TransferMessage::Cancel { generation }).await;
    });
    task.abort_handle()
}

fn spawn_deadline(
    sender: WeakSender<TransferMessage>,
    generation: u64,
    deadline: Deadline,
    duration: Duration,
) -> AbortHandle {
    let task = tokio::spawn(async move {
        sleep(duration).await;
        let Some(sender) = sender.upgrade() else {
            return;
        };
        let _result = sender
            .send(TransferMessage::Deadline {
                generation,
                deadline,
            })
            .await;
    });
    task.abort_handle()
}

fn abort_task(task: &mut Option<AbortHandle>) {
    if let Some(task) = task.take() {
        task.abort();
    }
}

async fn image_block_operation(
    zcl: &Sender<zcl::Message>,
    image: &ImageTransfer,
    context: RequestContext,
    block_request: ImageBlockRequest,
    range: ImageRange,
) -> OperationOutcome {
    let data = match read_image_range(image, range.offset, range.length).await {
        Ok(data) => data,
        Err(status) => {
            return transmit_request_error(
                zcl,
                context,
                <ImageBlockRequest as Command>::ID,
                status,
            )
            .await;
        }
    };

    let block = ImageBlock::try_new(block_request.image(), block_request.file_offset(), data)
        .expect("requested OTA blocks never exceed the client's u8 maximum data size");
    let response = ImageBlockResponse::new(ImageBlockResponsePayload::Success(block));
    let request = request(
        context.destination.into(),
        context.source_endpoint,
        OTA_PROFILE,
        Cluster::OtaUpgrade.as_u16(),
        response,
    );
    operation_outcome(
        transmit_reply(zcl, context.sequence_number, request).await,
        None,
    )
}

async fn image_page_operation(
    zcl: Sender<zcl::Message>,
    image: ImageTransfer,
    context: RequestContext,
    page_request: ImagePageRequest,
    range: ImageRange,
) -> OperationOutcome {
    let first_block = match read_image_range(&image, range.offset, range.length).await {
        Ok(data) => data,
        Err(status) => {
            return transmit_request_error(
                &zcl,
                context,
                <ImagePageRequest as Command>::ID,
                status,
            )
            .await;
        }
    };

    let image_id = page_request.image();
    let maximum_data_size = usize::from(page_request.maximum_data_size());
    let page_end = usize::try_from(page_request.file_offset())
        .unwrap_or(usize::MAX)
        .saturating_add(usize::from(page_request.page_size()))
        .min(image.len());
    let spacing = Duration::from_millis(u64::from(page_request.response_spacing()));
    let operation = PageTransfer {
        zcl,
        image,
        destination: context.destination,
        source_endpoint: context.source_endpoint,
        image_id,
        maximum_data_size,
        page_end,
        spacing,
        offset: usize::try_from(page_request.file_offset())
            .expect("validated OTA file offset fits usize"),
        sequence_number: context.sequence_number,
        block_data: first_block,
    };
    operation_outcome(operation.run().await, None)
}

async fn transmit_request_error(
    zcl: &Sender<zcl::Message>,
    context: RequestContext,
    request_command_id: u8,
    status: Status,
) -> OperationOutcome {
    let completion = (status == Status::Failure).then_some(Err(UpdateError::ImageTransfer));
    let request = default_response_request(context, request_command_id, status);
    operation_outcome(
        transmit_reply(zcl, context.sequence_number, request).await,
        completion,
    )
}

fn default_response_request(
    context: RequestContext,
    request_command_id: u8,
    status: Status,
) -> Request {
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
    request_from_unsequenced_frame(
        context.destination.into(),
        context.source_endpoint,
        OTA_PROFILE,
        Cluster::OtaUpgrade.as_u16(),
        frame,
    )
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

fn requested_range(
    target: FullAddress,
    image: &ImageTransfer,
    requested_image: ImageId,
    file_offset: u32,
    maximum_data_size: u8,
    request_node_address: Option<IeeeAddress>,
    page_size: Option<u16>,
) -> Result<ImageRange, Status> {
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
    Ok(ImageRange {
        offset,
        length: end - offset,
    })
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
