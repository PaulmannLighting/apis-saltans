use core::time::Duration;

use le_stream::{FromLeStream, ToLeStream};

use crate::types::Uint16;

const MILLIS_PER_DECISECOND: u64 = 100;

/// Type to represent a duration in 1/10ths of a second.
///
/// The inner type is guaranteed to not be `Uint16::NONE`.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    //serde(try_from = "Uint16", into = "Uint16")
)]
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, FromLeStream, ToLeStream,
)]
pub struct Deciseconds(Uint16);

impl Deciseconds {
    /// Create a new deciseconds value.
    #[must_use]
    pub fn new(deciseconds: u16) -> Option<Self> {
        deciseconds.try_into().map(Self).ok()
    }

    /// Get the inner value.
    #[must_use]
    pub const fn into_inner(self) -> Uint16 {
        self.0
    }
}

impl TryFrom<Uint16> for Deciseconds {
    type Error = Uint16;

    fn try_from(value: Uint16) -> Result<Self, Self::Error> {
        if value == Uint16::NONE {
            Err(value)
        } else {
            Ok(Self(value))
        }
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

impl From<Deciseconds> for Duration {
    fn from(value: Deciseconds) -> Self {
        Self::from_millis(
            u64::from(u16::try_from(value.0).expect("Inner value is guaranteed to not be NONE."))
                * MILLIS_PER_DECISECOND,
        )
    }
}
