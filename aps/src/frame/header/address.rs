#[cfg_attr(
    feature = "le-stream",
    derive(le_stream::FromLeStream, le_stream::ToLeStream)
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Address {
    destination_endpoint: Option<u8>,
    group_address: Option<u16>,
    cluster_id: Option<u16>,
    profile_id: Option<u16>,
    source_endpoint: Option<u8>,
}
