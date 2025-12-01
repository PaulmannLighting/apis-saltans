use le_stream::{FromLeStream, ToLeStream};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct Address {
    destination_endpoint: Option<u8>,
    #[expect(clippy::struct_field_names)]
    group_address: Option<u16>,
    cluster_id: Option<u16>,
    profile_id: Option<u16>,
    source_endpoint: Option<u8>,
}
