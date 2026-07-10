use std::marker::PhantomData;

use apis_saltans_core::destination::Device;
use apis_saltans_core::{ClusterSpecific, Direction, ExpectResponse};
use apis_saltans_zcl::global::read_attributes;
use apis_saltans_zcl::{Cluster, Command, ParseAttributeError, Readable, Scope, Scoped};
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
        ids: T,
    ) -> Result<Box<[ReadAttributeResult<T::Item>]>, Error>
    where
        T: IntoIterator<Item: Readable + Send, IntoIter: Send> + Send,
    {
        let response = self
            .communicate(device, ReadAttributesRequest::<T::Item>::new(ids))
            .await?;

        Ok(response.into())
    }
}

impl ReadAttributes for Coordinator {
    async fn read_attributes<T>(
        &self,
        device: Device,
        ids: T,
    ) -> Result<Box<[ReadAttributeResult<T::Item>]>, Error>
    where
        T: IntoIterator<Item: Readable + Send, IntoIter: Send> + Send,
    {
        self.zcl.read_attributes(device, ids).await
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

impl<T> Command for ReadAttributesRequest<T> {
    const ID: u8 = <read_attributes::Command as Command>::ID;
    const DIRECTION: Direction = <read_attributes::Command as Command>::DIRECTION;
    const PARSE_DIRECTION: apis_saltans_zcl::ParseDirection =
        <read_attributes::Command as Command>::PARSE_DIRECTION;
    const DISABLE_DEFAULT_RESPONSE: bool =
        <read_attributes::Command as Command>::DISABLE_DEFAULT_RESPONSE;
}

impl<T> Scoped for ReadAttributesRequest<T> {
    const SCOPE: Scope = Scope::Global;
}

impl<T> ToLeStream for ReadAttributesRequest<T> {
    type Iter = <read_attributes::Command as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        read_attributes::Command::new(self.attribute_ids).to_le_stream()
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
                scope: ReadAttributesRequest::<T>::SCOPE,
                direction: ReadAttributesRequest::<T>::DIRECTION,
                disable_default_response: ReadAttributesRequest::<T>::DISABLE_DEFAULT_RESPONSE,
                manufacturer_code: T::MANUFACTURER_CODE,
                command_id: ReadAttributesRequest::<T>::ID,
            },
            request.to_le_stream().collect(),
        )
    }
}
