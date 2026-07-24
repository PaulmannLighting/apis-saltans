use le_stream::ToLeStream;
use zb_core::{ClusterSpecific, ExpectResponse, Profiled};
use zb_zcl::global::read_attributes;
use zb_zcl::{Cluster, Command, Readable, Scoped};

use crate::zcl::{Metadata, Payload};

/// Global Read Attributes request scoped to one target cluster.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadAttributesRequest<T>(pub T);

impl<T> ExpectResponse<Cluster> for ReadAttributesRequest<T> {
    type Response = read_attributes::Response;
}

impl<T> From<ReadAttributesRequest<T>> for Payload
where
    T: IntoIterator<Item: Readable>,
{
    fn from(request: ReadAttributesRequest<T>) -> Self {
        Self::new(
            crate::aps::Metadata::new(
                <T::Item as Profiled>::PROFILE,
                <T::Item as ClusterSpecific>::ID,
            ),
            Metadata {
                scope: read_attributes::Command::SCOPE,
                direction: <read_attributes::Command as zb_zcl::Directed>::DIRECTION,
                disable_default_response: read_attributes::Command::DISABLE_DEFAULT_RESPONSE,
                manufacturer_code: <T::Item as Readable>::MANUFACTURER_CODE,
                command_id: read_attributes::Command::ID,
            },
            read_attributes::Command::new(request.0.into_iter().map(Into::into).collect())
                .to_le_stream()
                .collect(),
        )
    }
}
