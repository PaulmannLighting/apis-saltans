use std::fmt::Debug;

use bytes::Bytes;
use le_stream::ToLeStream;
use log::trace;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;
use zb_aps::TxOptions;
use zb_aps::apsde::{DataRequest, IndividualEndpoint};
use zb_core::{ClusterSpecific, Destination, Profiled};
use zb_zcl::{Cluster, Command, Directed, Scoped, UnsequencedFrame};

use crate::zcl::Message;
use crate::{CommunicationResponse, Coordinator, Error};

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
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the command cannot be queued or an acknowledged APS transmission
    /// fails.
    fn transmit(
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
