use le_stream::ToLeStream;
use tokio::sync::mpsc::Sender;
use zb_core::destination::Device;
use zb_core::{ClusterSpecific, ExpectResponse};
use zb_zcl::global::write_attributes;
use zb_zcl::{Cluster, Command, Scoped, Writable};

use crate::transceiver::zcl::{Handle, Message, Metadata, Payload};
use crate::{Coordinator, Error};

/// Result of writing an attribute.
pub type WriteAttributeResult = Result<u16, u16>;

/// Trait to write attributes to a device.
pub trait WriteAttributes {
    /// Write attributes to a device.
    ///
    /// # Returns
    ///
    /// Returns a [`Vec`] of [`Result`]s, where each [`Result`] contains the status of the write operation for each attribute.
    ///
    /// - `Ok(id)`: The attribute was successfully written.
    /// - `Err(id)`: The attribute could not be written.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the communication fails or if the response is not a valid [`Response`].
    fn write_attributes<T>(
        &self,
        device: Device,
        attributes: T,
    ) -> impl Future<Output = Result<Vec<WriteAttributeResult>, Error>> + Send
    where
        Self: Sync,
        T: IntoIterator<Item: Writable + Send, IntoIter: Send> + Send;
}

impl WriteAttributes for Sender<Message> {
    async fn write_attributes<T>(
        &self,
        device: Device,
        attributes: T,
    ) -> Result<Vec<WriteAttributeResult>, Error>
    where
        T: IntoIterator<Item: Writable + Send, IntoIter: Send> + Send,
    {
        let response = self
            .communicate(device, WriteAttributesRequest::new(attributes))
            .await?;

        Ok(response.into_iter().map(TryInto::try_into).collect())
    }
}

impl WriteAttributes for Coordinator {
    async fn write_attributes<T>(
        &self,
        device: Device,
        attributes: T,
    ) -> Result<Vec<WriteAttributeResult>, Error>
    where
        T: IntoIterator<Item: Writable + Send, IntoIter: Send> + Send,
    {
        self.zcl.write_attributes(device, attributes).await
    }
}

/// Global Write Attributes request scoped to one target cluster.
#[derive(Clone, Debug, Eq, PartialEq)]
struct WriteAttributesRequest<T> {
    records: Box<[write_attributes::Record]>,
    attribute: std::marker::PhantomData<T>,
}

impl<T> WriteAttributesRequest<T>
where
    T: Writable,
{
    fn new<I>(attributes: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            records: attributes.into_iter().map(Into::into).collect(),
            attribute: std::marker::PhantomData,
        }
    }
}

impl<T> ExpectResponse<Cluster> for WriteAttributesRequest<T> {
    type Response = write_attributes::Response;
}

impl<T> From<WriteAttributesRequest<T>> for Payload
where
    T: Writable,
{
    fn from(request: WriteAttributesRequest<T>) -> Self {
        Self::new(
            zb_hw::Metadata::new(T::PROFILE, <T as ClusterSpecific>::ID),
            Metadata {
                scope: write_attributes::Command::SCOPE,
                direction: <write_attributes::Command as zb_zcl::Directed>::DIRECTION,
                disable_default_response: write_attributes::Command::DISABLE_DEFAULT_RESPONSE,
                manufacturer_code: T::MANUFACTURER_CODE,
                command_id: write_attributes::Command::ID,
            },
            write_attributes::Command::new(request.records)
                .to_le_stream()
                .collect(),
        )
    }
}
