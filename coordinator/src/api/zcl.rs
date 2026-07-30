use std::fmt::Debug;

use bytes::Bytes;
use le_stream::ToLeStream;
use log::trace;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;
use zb_aps::TxOptions;
use zb_aps::apsde::{DataRequest, IndividualEndpoint};
use zb_core::{ClusterSpecific, Destination, Profiled};
use zb_zcl::global::default_response::DefaultResponse;
use zb_zcl::{Cluster, Command, Directed, Scoped, UnsequencedFrame};

use crate::zcl::Message;
use crate::{CommunicationResponse, Coordinator, Error, StatusExt};

const DEFAULT_TX_OPTIONS: TxOptions = TxOptions::ACKNOWLEDGED_TRANSMISSION;

/// A deferred typed ZCL response.
///
/// Awaiting this future completes the APS transmission, waits for the correlated ZCL frame, and
/// converts it to `T`.
pub type ZclResponse<T> = CommunicationResponse<Cluster, T>;

/// Construct a ZCL data request using a command's profile and cluster identifiers.
pub fn request<T>(
    destination: Destination,
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
    destination: Destination,
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
    destination: Destination,
    source_endpoint: IndividualEndpoint,
    profile_id: u16,
    cluster_id: u16,
    frame: UnsequencedFrame<Bytes>,
) -> DataRequest<UnsequencedFrame<Bytes>> {
    DataRequest::new(
        crate::aps::request_destination(destination),
        profile_id,
        cluster_id,
        source_endpoint,
        frame,
    )
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

    /// Send a ZCL command and wait for its typed response.
    ///
    /// The request destination must be one individual 16-bit NWK endpoint. The returned outer
    /// future queues the request and yields a [`ZclResponse`]. Await that response separately to
    /// complete APS transmission, receive the correlated ZCL response frame, and convert it.
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
    ) -> impl Future<Output = Result<ZclResponse<T>, Error>> + Send
    where
        T: TryFrom<Cluster, Error: Debug> + Send;
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
            let response = self.communicate::<DefaultResponse>(request).await?.await?;
            validate_default_response(command_id, &response)
        }
    }

    fn communicate<T>(
        &self,
        request: DataRequest<UnsequencedFrame<Bytes>>,
    ) -> impl Future<Output = Result<ZclResponse<T>, Error>> + Send
    where
        T: TryFrom<Cluster, Error: Debug> + Send,
    {
        let (response, result) = channel();

        async move {
            self.send(Message::Communicate { request, response })
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
    ) -> impl Future<Output = Result<ZclResponse<T>, Error>> + Send
    where
        T: TryFrom<Cluster, Error: Debug> + Send,
    {
        self.zcl.communicate(request)
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
    use zb_aps::apsde::IndividualEndpoint;
    use zb_core::destination::Device;
    use zb_core::endpoint::Application;
    use zb_core::{Endpoint, short_id};
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

    fn destination() -> zb_core::Destination {
        Device::new(
            short_id::Device::new(DEVICE_ID).expect("test device ID is valid"),
            Endpoint::Application(Application::MIN),
        )
        .into()
    }

    const fn source_endpoint() -> IndividualEndpoint {
        IndividualEndpoint::new(Endpoint::Application(Application::MIN))
            .expect("application endpoint is individual")
    }
}
