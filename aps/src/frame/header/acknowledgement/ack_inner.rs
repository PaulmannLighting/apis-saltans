use le_stream::{FromLeStream, ToLeStream};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct AckInner {
    destination: u8,
    cluster_id: u16,
    profile_id: u16,
    source: u8,
}
