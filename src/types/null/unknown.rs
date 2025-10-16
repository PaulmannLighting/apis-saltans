use le_stream::derive::{FromLeStream, ToLeStream};

/// The `Unknown` data type, short `unk`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
pub struct Unknown;


impl From<()> for Unknown {
    fn from((): ()) -> Self {
        Self
    }
}

impl From<Unknown> for () {
    fn from(_: Unknown) -> Self {}
}
