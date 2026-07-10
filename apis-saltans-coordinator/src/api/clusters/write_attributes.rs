use apis_saltans_core::destination::Device;
use apis_saltans_core::{ClusterSpecific, Direction, ExpectResponse};
use apis_saltans_zcl::global::write_attributes;
use apis_saltans_zcl::{Cluster, Command, Scope, Scoped, Writable};
use le_stream::ToLeStream;
use tokio::sync::mpsc::Sender;

use crate::transceiver::zcl::{Handle, Message, Metadata, Payload};
use crate::{Coordinator, Error};

/// Result of writing an attribute.
pub type WriteAttributeResult = Result<u16, u16>;

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

impl<T> Command for WriteAttributesRequest<T> {
    const ID: u8 = <write_attributes::Command as Command>::ID;
    const DIRECTION: Direction = <write_attributes::Command as Command>::DIRECTION;
    const PARSE_DIRECTION: apis_saltans_zcl::ParseDirection =
        <write_attributes::Command as Command>::PARSE_DIRECTION;
    const DISABLE_DEFAULT_RESPONSE: bool =
        <write_attributes::Command as Command>::DISABLE_DEFAULT_RESPONSE;
}

impl<T> Scoped for WriteAttributesRequest<T> {
    const SCOPE: Scope = Scope::Global;
}

impl<T> ToLeStream for WriteAttributesRequest<T> {
    type Iter = <write_attributes::Command as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        write_attributes::Command::new(self.records).to_le_stream()
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
            apis_saltans_hw::Metadata::new(T::PROFILE, <T as ClusterSpecific>::ID),
            Metadata {
                scope: WriteAttributesRequest::<T>::SCOPE,
                direction: WriteAttributesRequest::<T>::DIRECTION,
                disable_default_response: WriteAttributesRequest::<T>::DISABLE_DEFAULT_RESPONSE,
                manufacturer_code: T::MANUFACTURER_CODE,
                command_id: WriteAttributesRequest::<T>::ID,
            },
            request.to_le_stream().collect(),
        )
    }
}

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
            .communicate(device, WriteAttributesRequest::<T::Item>::new(attributes))
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
