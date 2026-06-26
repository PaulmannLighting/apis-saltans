use core::time::Duration;

use crate::types::Uint16;

const MILLIS_PER_DECISECOND: u64 = 100;

/// Type to represent a duration in 1/10ths of a second.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Deciseconds<T>(T);

impl<T> Deciseconds<T> {
    /// Create a new [`Deciseconds`] from the given value.
    #[must_use]
    pub const fn new(value: T) -> Self {
        Self(value)
    }

    /// Get the inner value.
    #[must_use]
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deciseconds<T>
where
    T: Copy,
{
    /// Return a copy of the inner value.
    #[must_use]
    pub const fn copy_inner(self) -> T {
        self.0
    }
}

impl TryFrom<Duration> for Deciseconds<u16> {
    type Error = Duration;

    fn try_from(duration: Duration) -> Result<Self, Self::Error> {
        Ok(Self(
            u16::try_from(duration.as_millis() / u128::from(MILLIS_PER_DECISECOND))
                .map_err(|_| duration)?,
        ))
    }
}

impl TryFrom<Duration> for Deciseconds<Uint16> {
    type Error = Duration;

    fn try_from(duration: Duration) -> Result<Self, Self::Error> {
        Ok(Self(
            Deciseconds::<u16>::try_from(duration)?
                .0
                .try_into()
                .map_err(|()| duration)?,
        ))
    }
}

impl From<Deciseconds<u16>> for Duration {
    fn from(deciseconds: Deciseconds<u16>) -> Self {
        Self::from_millis(u64::from(deciseconds.0) * MILLIS_PER_DECISECOND)
    }
}

impl From<Deciseconds<Uint16>> for Duration {
    fn from(value: Deciseconds<Uint16>) -> Self {
        Self::from_millis(
            u64::from(Option::<u16>::from(value.0).unwrap_or_default()) * MILLIS_PER_DECISECOND,
        )
    }
}
