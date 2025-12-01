/// Extended header.
/// TODO: Implement fields.
#[cfg_attr(
    feature = "le-stream",
    derive(le_stream::FromLeStream, le_stream::ToLeStream)
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Extended {}
