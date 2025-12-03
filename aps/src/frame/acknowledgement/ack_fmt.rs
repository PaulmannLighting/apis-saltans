use le_stream::{FromLeStream, ToLeStream};

/// Additional ack frame format information.
///
/// This structure is present in acknowledgment frames when the `ack format` bit
/// in the control field is *not* set.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct AckFmt {
    destination: u8,
    cluster_id: u16,
    profile_id: u16,
    source: u8,
}
