use std::time::Duration;

use tokio::time::sleep;

/// Utility struct to facilitate retrying operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Retry {
    max: usize,
    delay: Duration,
}

impl Retry {
    /// Create a new instance of `Retry`.
    #[must_use]
    pub const fn new(max: usize, delay: Duration) -> Self {
        Self { max, delay }
    }

    /// Return `true` as long as the number of retries is less than the maximum.
    #[must_use]
    pub async fn retry(&self, retries: &mut usize) -> bool {
        if *retries > self.max {
            return false;
        }

        if *retries > 0 {
            sleep(self.delay).await;
        }

        if let Some(next) = retries.checked_add(1) {
            *retries = next;
        } else {
            return false;
        }

        true
    }
}
