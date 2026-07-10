use apis_saltans_core::destination::Device;
use apis_saltans_core::{ClusterSpecific, Direction, ExpectResponse, Profile, Profiled};
use apis_saltans_zcl::{Cluster, Command, ParseAttributeError, ParseDirection, Readable};
use le_stream::ToLeStream;
use tokio::sync::mpsc::Sender;

use crate::Error;
use crate::transceiver::zcl::{Handle, Message};

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
    ) -> impl Future<
        Output = Result<<<T as IntoIterator>::Item as ExpectResponse<Cluster>>::Response, Error>,
    > + Send
    where
        Self: Sync,
        T: IntoIterator<
                Item: Readable
                          + Command
                          + ExpectResponse<Cluster>
                          + ToLeStream
                          + Into<Cluster>
                          + Send,
                IntoIter: Send,
            > + Send;
}

impl ReadAttributes for Sender<Message> {
    async fn read_attributes<T>(
        &self,
        device: Device,
        ids: T,
    ) -> Result<<<T as IntoIterator>::Item as ExpectResponse<Cluster>>::Response, Error>
    where
        T: IntoIterator<
                Item: Readable
                          + Command
                          + ExpectResponse<Cluster>
                          + ToLeStream
                          + Into<Cluster>
                          + Send,
                IntoIter: Send,
            > + Send,
    {
        self.communicate(device, ReadAttributesRequest::from_iter(ids))
            .await
    }
}

struct ReadAttributesRequest<T> {
    attributes: Box<[T]>,
}

impl<T> FromIterator<T> for ReadAttributesRequest<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            attributes: iter.into_iter().collect(),
        }
    }
}

impl<T> ClusterSpecific for ReadAttributesRequest<T>
where
    T: ClusterSpecific,
{
    const ID: u16 = T::ID;
}

impl<T> Profiled for ReadAttributesRequest<T>
where
    T: Profiled,
{
    const PROFILE: Profile = T::PROFILE;
}

impl<T> apis_saltans_zcl::Command for ReadAttributesRequest<T>
where
    T: apis_saltans_zcl::Command,
{
    const ID: u8 = T::ID;
    const DIRECTION: Direction = T::DIRECTION;
    const PARSE_DIRECTION: ParseDirection = T::PARSE_DIRECTION;
    const MANUFACTURER_CODE: Option<u16> = T::MANUFACTURER_CODE;
    const DISABLE_DEFAULT_RESPONSE: bool = T::DISABLE_DEFAULT_RESPONSE;
}

impl<T, U> ExpectResponse<U> for ReadAttributesRequest<T>
where
    T: ExpectResponse<U> + Into<U>,
{
    type Response = T::Response;
}

impl<T> ToLeStream for ReadAttributesRequest<T>
where
    T: ToLeStream,
{
    type Iter = <Box<[T]> as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        self.attributes.to_le_stream()
    }
}
