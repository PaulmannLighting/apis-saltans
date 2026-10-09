use std::fmt::Debug;

use bytes::Bytes;
use le_stream::ToLeStream;
use log::trace;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;
use zb_aps::TxOptions;
use zb_aps::apsde::{DataRequest, IndividualEndpoint, RequestDestination};
use zb_core::{ClusterSpecific, Profiled};
use zb_zcl::global::default_response::DefaultResponse;
use zb_zcl::{Cluster, Command, Directed, Scoped, UnsequencedFrame};

pub use self::outcome::{DefaultResponsePolicy, ZclOutcome};
pub use self::request_policy::ZclRequestPolicy;
use crate::zcl::Message;
use crate::{CommunicationResponse, Coordinator, Error, StatusExt};

mod outcome;
mod request_policy;

const DEFAULT_TX_OPTIONS: TxOptions = TxOptions::ACKNOWLEDGED_TRANSMISSION;

/// A deferred typed ZCL response.
///
/// Awaiting this future completes the APS transmission, waits for the correlated ZCL frame, and
/// returns a [`ZclOutcome<T>`] containing the specific response or a permitted Default Response.
pub type ZclResponse<T> = CommunicationResponse<Cluster, ZclOutcome<T>>;

/// Defines which command headers can satisfy a typed ZCL response.
///
/// Individual command types implement this automatically through `Command` and `Scoped`.
/// Response enums accepting multiple command forms can implement this trait explicitly.
/// Implementations should match both scope and command ID and must not accept unrelated commands.
pub trait ZclResponseType {
    /// Whether this header describes an accepted response command.
    fn matches_response(header: zb_zcl::Header) -> bool;
}

impl<T> ZclResponseType for T
where
    T: Command + Scoped,
{
    fn matches_response(header: zb_zcl::Header) -> bool {
        header.control().typ() == Ok(T::SCOPE) && header.command_id() == T::ID
    }
}

/// Construct a ZCL data request using a command's profile and cluster identifiers.
pub fn request<T>(
    destination: RequestDestination,
    source_endpoint: IndividualEndpoint,
    command: T,
) -> DataRequest<UnsequencedFrame<Bytes>>
where
    T: ClusterSpecific + Command + Directed + Profiled + Scoped + ToLeStream,
{
    request_with_ids(
        destination,
        source_endpoint,
        T::PROFILE.as_u16(),
        <T as ClusterSpecific>::ID,
        UnsequencedFrame::from_command(command),
    )
}

/// Construct a response-free ZCL data request.
///
/// This sets the disable-default-response flag explicitly. Use [`request`] with
/// [`Zcl::communicate`] when the command has a cluster-specific response.
pub fn request_without_response<T>(
    destination: RequestDestination,
    source_endpoint: IndividualEndpoint,
    command: T,
) -> DataRequest<UnsequencedFrame<Bytes>>
where
    T: ClusterSpecific + Command + Directed + Profiled + Scoped + ToLeStream,
{
    request(destination, source_endpoint, command)
        .map_asdu(|frame| frame.with_disable_default_response(true))
}

/// Construct a ZCL data request using explicitly selected profile and cluster identifiers.
pub const fn request_with_ids(
    destination: RequestDestination,
    source_endpoint: IndividualEndpoint,
    profile_id: u16,
    cluster_id: u16,
    frame: UnsequencedFrame<Bytes>,
) -> DataRequest<UnsequencedFrame<Bytes>> {
    DataRequest::new(destination, profile_id, cluster_id, source_endpoint, frame)
        .with_tx_options(DEFAULT_TX_OPTIONS)
}

/// Trait for sending ZCL commands.
///
/// `Coordinator` implements this trait directly. Every operation accepts a complete
/// [`DataRequest`], so callers explicitly select the local source endpoint together with the APS
/// destination, profile, cluster, and transmission options.
pub trait Zcl {
    /// Send a ZCL command without waiting for an application-level response.
    ///
    /// Use this for cluster commands that are transmitted as commands or group/broadcast messages.
    /// An individual unicast must disable default responses; use
    /// [`Self::communicate_default`] when a Default Response is expected.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the command cannot be queued or an acknowledged APS transmission
    /// fails.
    fn transmit(
        &self,
        request: DataRequest<UnsequencedFrame<Bytes>>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Send an individual ZCL unicast and validate its Default Response.
    ///
    /// This method enables default responses on the transmitted frame, waits for APS completion,
    /// verifies that the response names the transmitted command, and returns a ZCL status error
    /// when the remote device rejects the command.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request cannot be queued, does not address an individual
    /// network endpoint, cannot be transmitted, does not receive a valid matching Default
    /// Response, or carries an unsuccessful ZCL status.
    fn communicate_default(
        &self,
        request: DataRequest<UnsequencedFrame<Bytes>>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Send a typed cluster command using the policy derived from its request payload.
    ///
    /// # Errors
    ///
    /// Returns the same queueing and response errors as [`Self::communicate`].
    fn communicate_command<C, T>(
        &self,
        destination: RequestDestination,
        source_endpoint: IndividualEndpoint,
        command: C,
    ) -> impl Future<Output = Result<ZclResponse<T>, Error>> + Send
    where
        C: ZclRequestPolicy + ClusterSpecific + Command + Directed + Profiled + Scoped + ToLeStream,
        T: ZclResponseType + TryFrom<Cluster, Error: Debug> + Send,
    {
        let policy = command.default_response_policy();
        self.communicate(request(destination, source_endpoint, command), policy)
    }

    /// Send a ZCL command and wait for its typed response.
    ///
    /// The request destination must be one individual 16-bit NWK endpoint. The returned outer
    /// future queues the request and yields a [`ZclResponse`]. Await that response separately to
    /// complete APS transmission, receive the correlated ZCL response frame, and convert it.
    /// `T` supplies the accepted command scope and ID through [`ZclResponseType`]. Unrelated
    /// frames leave the request pending. Default Responses must additionally name the original
    /// command. The explicit request policy controls their treatment, independently of `T`:
    /// [`DefaultResponsePolicy::DefaultAllowed`] returns [`ZclOutcome::Default`] for any status;
    /// [`DefaultResponsePolicy::SpecificRequired`] returns [`Error::Zcl`] for device rejection
    /// or [`Error::UnexpectedDefaultResponse`] for success without the required typed response.
    /// A matching Default Response always resolves this one-response exchange immediately,
    /// even when the outgoing disable-default-response bit is set. For payload-dependent rules,
    /// derive the policy before encoding or use [`Self::communicate_command`].
    ///
    /// # Errors
    ///
    /// The outer future returns an [`Error`] if the request cannot be queued or does not address an
    /// individual NWK endpoint. Awaiting the returned [`ZclResponse`] returns an [`Error`] if APS
    /// transmission or protocol reception fails, or if the raw frame cannot be converted into
    /// `T`.
    fn communicate<T>(
        &self,
        request: DataRequest<UnsequencedFrame<Bytes>>,
        default_response_policy: DefaultResponsePolicy,
    ) -> impl Future<Output = Result<ZclResponse<T>, Error>> + Send
    where
        T: ZclResponseType + TryFrom<Cluster, Error: Debug> + Send;
}

impl Zcl for Sender<Message> {
    fn transmit(
        &self,
        request: DataRequest<UnsequencedFrame<Bytes>>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let destination = request.destination();
        let (response, result) = channel();
        trace!("Sending ZCL message to {destination:?}");
        async move {
            self.send(Message::Transmit { request, response }).await?;
            result.await??.await?;
            Ok(())
        }
    }

    fn communicate_default(
        &self,
        request: DataRequest<UnsequencedFrame<Bytes>>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let command_id = request.asdu().header().command_id();
        let request = request.map_asdu(|frame| frame.with_disable_default_response(false));

        async move {
            let response = self
                .communicate::<DefaultResponse>(request, DefaultResponsePolicy::DefaultAllowed)
                .await?
                .await?;
            let response = match response {
                ZclOutcome::Default(response) | ZclOutcome::Specific(response) => response,
            };
            validate_default_response(command_id, &response)
        }
    }

    fn communicate<T>(
        &self,
        request: DataRequest<UnsequencedFrame<Bytes>>,
        default_response_policy: DefaultResponsePolicy,
    ) -> impl Future<Output = Result<ZclResponse<T>, Error>> + Send
    where
        T: ZclResponseType + TryFrom<Cluster, Error: Debug> + Send,
    {
        let (response, result) = channel();
        let expected = crate::zcl::ResponseExpectation::new::<T>(
            request.asdu().header().command_id(),
            default_response_policy,
        );

        async move {
            self.send(Message::Communicate {
                request,
                expected,
                response,
            })
            .await?;

            Ok(result.await??.into())
        }
    }
}

impl Zcl for Coordinator {
    fn transmit(
        &self,
        request: DataRequest<UnsequencedFrame<Bytes>>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        self.zcl.transmit(request)
    }

    fn communicate_default(
        &self,
        request: DataRequest<UnsequencedFrame<Bytes>>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        self.zcl.communicate_default(request)
    }

    fn communicate<T>(
        &self,
        request: DataRequest<UnsequencedFrame<Bytes>>,
        default_response_policy: DefaultResponsePolicy,
    ) -> impl Future<Output = Result<ZclResponse<T>, Error>> + Send
    where
        T: ZclResponseType + TryFrom<Cluster, Error: Debug> + Send,
    {
        self.zcl.communicate(request, default_response_policy)
    }
}

fn validate_default_response(command_id: u8, response: &DefaultResponse) -> Result<(), Error> {
    if response.command_id() != command_id {
        return Err(Error::InvalidResponseType(format!(
            "Default Response names command {:#04X}, expected {command_id:#04X}",
            response.command_id()
        )));
    }
    zb_zcl::Status::try_from(response.status()).ensure_success()
}

#[cfg(test)]
mod tests {
    use zb_aps::apsde::{
        IndividualEndpoint, NetworkAddress, NetworkDestination, RequestDestination,
    };
    use zb_core::Endpoint;
    use zb_core::endpoint::Application;
    use zb_zcl::Command;
    use zb_zcl::global::default_response::DefaultResponse;
    use zb_zcl::on_off::On;

    use super::{request_without_response, validate_default_response};
    use crate::Error;

    const DEVICE_ID: u16 = 0x1234;
    const OTHER_COMMAND_ID: u8 = 0x02;

    #[test]
    fn response_free_request_disables_default_responses() {
        let request = request_without_response(destination(), source_endpoint(), On);

        assert!(request.asdu().header().control().disable_default_response());
    }

    #[test]
    fn validates_a_successful_default_response() {
        let response = DefaultResponse::new(<On as Command>::ID, zb_zcl::Status::Success.into());

        assert!(validate_default_response(<On as Command>::ID, &response).is_ok());
    }

    #[test]
    fn rejects_a_default_response_for_another_command() {
        let response = DefaultResponse::new(OTHER_COMMAND_ID, zb_zcl::Status::Success.into());

        assert!(matches!(
            validate_default_response(<On as Command>::ID, &response),
            Err(Error::InvalidResponseType(_))
        ));
    }

    #[test]
    fn returns_an_unsuccessful_default_response_status() {
        let response = DefaultResponse::new(<On as Command>::ID, zb_zcl::Status::Failure.into());

        assert!(matches!(
            validate_default_response(<On as Command>::ID, &response),
            Err(Error::Zcl(Ok(zb_zcl::Status::Failure)))
        ));
    }

    #[test]
    fn communication_carries_typed_response_expectation_to_actor() {
        use std::future::Future;
        use std::pin::pin;
        use std::task::{Context, Poll, Waker};

        use zb_zcl::global::read_attributes;
        use zb_zcl::{Frame, Header, Scope};

        use super::{Zcl, request};
        use crate::zcl::Message;

        const SEQUENCE: u8 = 7;
        const CHANNEL_CAPACITY: usize = 1;
        const UNRELATED_DEFAULT: [u8; 5] = [0x18, SEQUENCE, 0x0b, 0x00, 0x00];
        const EXPECTED_DEFAULT: [u8; 5] = [0x18, SEQUENCE, 0x0b, 0x01, 0x00];
        let (sender, mut messages) = tokio::sync::mpsc::channel(CHANNEL_CAPACITY);
        let mut response = pin!(sender.communicate::<DefaultResponse>(
            request(destination(), source_endpoint(), On),
            super::DefaultResponsePolicy::DefaultAllowed
        ));
        let mut context = Context::from_waker(Waker::noop());
        assert!(matches!(
            response.as_mut().poll(&mut context),
            Poll::Pending
        ));
        let Message::Communicate { expected, .. } = messages.try_recv().unwrap() else {
            panic!("expected a ZCL communication request");
        };
        let frame = Frame::parse(
            zb_core::Cluster::OnOff.as_u16(),
            EXPECTED_DEFAULT.into_iter(),
        )
        .unwrap();
        assert!(matches!(
            expected.classify(&frame),
            crate::zcl::ResponseMatch::Expected
        ));
        let frame = Frame::parse(
            zb_core::Cluster::OnOff.as_u16(),
            UNRELATED_DEFAULT.into_iter(),
        )
        .unwrap();
        assert!(matches!(
            expected.classify(&frame),
            crate::zcl::ResponseMatch::Unrelated
        ));
        let unrelated = Frame::new(
            Header::new(
                Scope::Global,
                zb_core::Direction::ServerToClient,
                false,
                None,
                SEQUENCE,
                <read_attributes::Response as Command>::ID,
            ),
            frame.into_payload(),
        );
        assert!(matches!(
            expected.classify(&unrelated),
            crate::zcl::ResponseMatch::Unrelated
        ));
    }

    #[test]
    fn typed_requests_accept_error_defaults_even_when_default_responses_are_disabled() {
        use std::future::Future;
        use std::pin::pin;
        use std::task::{Context, Poll, Waker};

        use zb_zcl::Frame;
        use zb_zcl::global::read_attributes;

        use super::{Zcl, request_with_ids};
        use crate::zcl::{Message, ResponseMatch};

        const CHANNEL_CAPACITY: usize = 1;
        const FAILURE_RESPONSE: [u8; 5] = [0x18, 0x07, 0x0b, 0x00, 0x01];
        for disabled in [false, true] {
            let (sender, mut messages) = tokio::sync::mpsc::channel(CHANNEL_CAPACITY);
            let frame = zb_zcl::UnsequencedFrame::from_command(read_attributes::Command::new(
                Box::default(),
            ))
            .with_disable_default_response(disabled);
            let request = request_with_ids(
                destination(),
                source_endpoint(),
                zb_core::Profile::ZigbeeHomeAutomation.as_u16(),
                zb_core::Cluster::OnOff.as_u16(),
                frame,
            );
            let mut response = pin!(sender.communicate::<read_attributes::Response>(
                request,
                super::DefaultResponsePolicy::SpecificRequired
            ));
            let mut context = Context::from_waker(Waker::noop());
            assert!(matches!(
                response.as_mut().poll(&mut context),
                Poll::Pending
            ));
            let Message::Communicate {
                expected, request, ..
            } = messages.try_recv().unwrap()
            else {
                panic!("expected a ZCL communication request");
            };
            assert_eq!(
                request.asdu().header().control().disable_default_response(),
                disabled
            );
            let frame = Frame::parse(
                zb_core::Cluster::OnOff.as_u16(),
                FAILURE_RESPONSE.into_iter(),
            )
            .unwrap();
            assert_eq!(
                expected.classify(&frame),
                ResponseMatch::Rejected(Ok(zb_zcl::Status::Failure))
            );
        }
    }

    fn destination() -> RequestDestination {
        NetworkDestination::new(
            NetworkAddress::new(DEVICE_ID).expect("test device ID is valid"),
            source_endpoint(),
        )
        .into()
    }

    const fn source_endpoint() -> IndividualEndpoint {
        IndividualEndpoint::new(Endpoint::Application(Application::MIN))
            .expect("application endpoint is individual")
    }
}
