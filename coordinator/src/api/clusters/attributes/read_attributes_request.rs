use bytes::Bytes;
use le_stream::ToLeStream;
use zb_zcl::global::read_attributes;
use zb_zcl::{Command, Readable, Scoped, UnsequencedFrame, UnsequencedHeader};

/// Construct a global Read Attributes frame scoped to one target cluster.
pub fn frame<T>(attributes: T) -> UnsequencedFrame<Bytes>
where
    T: IntoIterator<Item: Readable>,
{
    UnsequencedFrame::new(
        UnsequencedHeader::new(
            read_attributes::Command::SCOPE,
            <read_attributes::Command as zb_zcl::Directed>::DIRECTION,
            read_attributes::Command::DISABLE_DEFAULT_RESPONSE,
            <T::Item as Readable>::MANUFACTURER_CODE,
            read_attributes::Command::ID,
        ),
        read_attributes::Command::new(attributes.into_iter().map(Into::into).collect())
            .to_le_stream()
            .collect(),
    )
}
