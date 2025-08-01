use le_stream::derive::{FromLeStream, ToLeStream};

/// The `No data` data type, short `noddata`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
pub struct NoData;

impl From<()> for NoData {
    fn from((): ()) -> Self {
        Self
    }
}

impl From<NoData> for () {
    fn from(_: NoData) -> Self {}
}
