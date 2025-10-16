use le_stream::derive::{FromLeStream, ToLeStream};

/// The `No data` data type, short `nodata`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
pub struct NoData;

impl NoData {
    /// Crate an arbitrary option from `NoData`.
    #[must_use]
    pub const fn into_option<T>(self) -> Option<T> {
        None
    }
}

impl From<()> for NoData {
    fn from((): ()) -> Self {
        Self
    }
}

impl From<NoData> for () {
    fn from(_: NoData) -> Self {}
}
