use le_stream::{FromLeStream, ToLeStream};

/// Extended header.
/// TODO: Implement fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct Extended {}
