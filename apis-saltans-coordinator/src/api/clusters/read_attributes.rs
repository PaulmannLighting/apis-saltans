use std::marker::PhantomData;

use apis_saltans_core::destination::Device;
use apis_saltans_core::{ClusterSpecific, ExpectResponse};
use apis_saltans_zcl::global::read_attributes;
use apis_saltans_zcl::{Cluster, Command, ParseAttributeError, Readable, Scoped};
use le_stream::ToLeStream;
use tokio::sync::mpsc::Sender;

use crate::transceiver::zcl::{Handle, Message, Metadata, Payload};
use crate::{Coordinator, Error};

/// Result of reading an attribute.
pub type ReadAttributeResult<T> = Result<<T as Readable>::Attribute, ParseAttributeError<T>>;

/// Trait for reading attributes.
pub trait ReadAttributes {
    /// Read attributes from a device.
    ///
    /// # Errors
    ///
    /// Returns an [Error] if the communication fails or if the response is not a valid [`Response`].
    fn read_attributes<T>(
        &self,
        device: Device,
        attributes: T,
    ) -> impl Future<Output = Result<Box<[ReadAttributeResult<T::Item>]>, Error>> + Send
    where
        Self: Sync,
        T: IntoIterator<Item: Readable + Send, IntoIter: Send> + Send;
}

impl ReadAttributes for Sender<Message> {
    async fn read_attributes<T>(
        &self,
        device: Device,
        attributes: T,
    ) -> Result<Box<[ReadAttributeResult<T::Item>]>, Error>
    where
        T: IntoIterator<Item: Readable + Send, IntoIter: Send> + Send,
    {
        let response = self
            .communicate(device, ReadAttributesRequest::new(attributes))
            .await?;

        Ok(response.into())
    }
}

impl ReadAttributes for Coordinator {
    async fn read_attributes<T>(
        &self,
        device: Device,
        attributes: T,
    ) -> Result<Box<[ReadAttributeResult<T::Item>]>, Error>
    where
        T: IntoIterator<Item: Readable + Send, IntoIter: Send> + Send,
    {
        self.zcl.read_attributes(device, attributes).await
    }
}

/// Global Read Attributes request scoped to one target cluster.
#[derive(Clone, Debug, Eq, PartialEq)]
struct ReadAttributesRequest<T> {
    attribute_ids: Box<[u16]>,
    attribute: PhantomData<T>,
}

impl<T> ReadAttributesRequest<T>
where
    T: Readable,
{
    fn new<I>(attributes: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            attribute_ids: attributes.into_iter().map(Into::into).collect(),
            attribute: PhantomData,
        }
    }
}

impl<T> ExpectResponse<Cluster> for ReadAttributesRequest<T> {
    type Response = read_attributes::Response;
}

impl<T> From<ReadAttributesRequest<T>> for Payload
where
    T: Readable,
{
    fn from(request: ReadAttributesRequest<T>) -> Self {
        Self::new(
            apis_saltans_hw::Metadata::new(T::PROFILE, <T as ClusterSpecific>::ID),
            Metadata {
                scope: read_attributes::Command::SCOPE,
                direction: read_attributes::Command::DIRECTION,
                disable_default_response: read_attributes::Command::DISABLE_DEFAULT_RESPONSE,
                manufacturer_code: T::MANUFACTURER_CODE,
                command_id: read_attributes::Command::ID,
            },
            read_attributes::Command::new(request.attribute_ids)
                .to_le_stream()
                .collect(),
        )
    }
}
