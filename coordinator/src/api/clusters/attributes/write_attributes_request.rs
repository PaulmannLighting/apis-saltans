use le_stream::ToLeStream;
use zb_core::{ClusterSpecific, ExpectResponse, Profiled};
use zb_zcl::global::write_attributes;
use zb_zcl::{Cluster, Command, Scoped, Writable};

use crate::zcl::{Metadata, Payload};

/// Global Write Attributes request scoped to one target cluster.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WriteAttributesRequest<T>(pub T);

impl<T> ExpectResponse<Cluster> for WriteAttributesRequest<T> {
    type Response = write_attributes::Response;
}

impl<T> From<WriteAttributesRequest<T>> for Payload
where
    T: IntoIterator<Item: Writable>,
{
    fn from(request: WriteAttributesRequest<T>) -> Self {
        Self::new(
            zb_hw::Metadata::new(
                <T::Item as Profiled>::PROFILE,
                <T::Item as ClusterSpecific>::ID,
            ),
            Metadata {
                scope: write_attributes::Command::SCOPE,
                direction: <write_attributes::Command as zb_zcl::Directed>::DIRECTION,
                disable_default_response: write_attributes::Command::DISABLE_DEFAULT_RESPONSE,
                manufacturer_code: <T::Item as Writable>::MANUFACTURER_CODE,
                command_id: write_attributes::Command::ID,
            },
            write_attributes::Command::new(request.0.into_iter().map(Into::into).collect())
                .to_le_stream()
                .collect(),
        )
    }
}
