#[cfg_attr(
    feature = "le-stream",
    derive(le_stream::FromLeStream, le_stream::ToLeStream)
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct AckInner {
    destination: u8,
    cluster_id: u16,
    profile_id: u16,
    source: u8,
}
