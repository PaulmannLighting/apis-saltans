use bytes::Bytes;
use le_stream::ToLeStream;
use zb_zcl::global::write_attributes;
use zb_zcl::{Command, Scoped, UnsequencedFrame, UnsequencedHeader, Writable};

/// Construct a global Write Attributes frame scoped to one target cluster.
pub fn frame<T>(attributes: T) -> UnsequencedFrame<Bytes>
where
    T: IntoIterator<Item: Writable>,
{
    UnsequencedFrame::new(
        UnsequencedHeader::new(
            write_attributes::Command::SCOPE,
            <write_attributes::Command as zb_zcl::Directed>::DIRECTION,
            write_attributes::Command::DISABLE_DEFAULT_RESPONSE,
            <T::Item as Writable>::MANUFACTURER_CODE,
            write_attributes::Command::ID,
        ),
        write_attributes::Command::new(attributes.into_iter().map(Into::into).collect())
            .to_le_stream()
            .collect(),
    )
}
