use std::time::Duration;

/// Utility struct to facilitate retrying operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Retry {
    max: u32,
    delay: Duration,
}

impl Retry {
    /// Create a new instance of `Retry`.
    #[must_use]
    pub const fn new(max: u32, delay: Duration) -> Self {
        Self { max, delay }
    }

    /// Return `true` as long as the number of retries is less than the maximum.
    #[must_use]
    pub async fn retry(&self, retries: &mut u32) -> Option<Duration> {
        if *retries > self.max {
            return None;
        }

        let delay = self.delay * *retries;

        if let Some(next) = retries.checked_add(1) {
            *retries = next;
        } else {
            return None;
        }

        Some(delay)
    }
}
