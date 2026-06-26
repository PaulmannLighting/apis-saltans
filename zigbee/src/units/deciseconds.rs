use core::time::Duration;

use le_stream::{FromLeStream, ToLeStream};

use crate::types::Uint16;

const MILLIS_PER_DECISECOND: u64 = 100;

/// Type to represent a duration in 1/10ths of a second.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, FromLeStream, ToLeStream,
)]
pub struct Deciseconds(Uint16);

impl Deciseconds {
    /// Get the inner value.
    #[must_use]
    pub const fn into_inner(self) -> Uint16 {
        self.0
    }
}

impl TryFrom<Duration> for Deciseconds {
    type Error = Duration;

    fn try_from(duration: Duration) -> Result<Self, Self::Error> {
        u16::try_from(duration.as_millis() / u128::from(MILLIS_PER_DECISECOND))
            .map_err(|_| duration)?
            .try_into()
            .map_err(|()| duration)
            .map(Self)
    }
}

impl TryFrom<Deciseconds> for Duration {
    type Error = Deciseconds;

    fn try_from(value: Deciseconds) -> Result<Self, Self::Error> {
        u16::try_from(value.0)
            .map_err(|()| value)
            .map(u64::from)
            .map(|deciseconds| deciseconds * MILLIS_PER_DECISECOND)
            .map(Self::from_millis)
    }
}
