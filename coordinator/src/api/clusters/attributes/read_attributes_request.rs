use std::marker::PhantomData;

use le_stream::ToLeStream;
use zb_core::{ClusterSpecific, ExpectResponse};
use zb_zcl::global::read_attributes;
use zb_zcl::{Cluster, Command, Readable, Scoped};

use crate::transceiver::zcl::{Metadata, Payload};

/// Global Read Attributes request scoped to one target cluster.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ReadAttributesRequest<T> {
    attribute_ids: Box<[u16]>,
    attribute: PhantomData<T>,
}

impl<T> ReadAttributesRequest<T>
where
    T: Readable,
{
    pub(super) fn new<I>(attributes: I) -> Self
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
            zb_hw::Metadata::new(T::PROFILE, <T as ClusterSpecific>::ID),
            Metadata {
                scope: read_attributes::Command::SCOPE,
                direction: <read_attributes::Command as zb_zcl::Directed>::DIRECTION,
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
