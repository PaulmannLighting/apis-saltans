use std::fmt::{Debug, Formatter};

#[cfg(test)]
const TEST_GENERATION: u64 = 0;

/// Coordinator-private identity for lifecycle messages associated with one protocol transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Token<K> {
    key: K,
    generation: u64,
}

impl<K> Token<K>
where
    K: Copy,
{
    /// Create a lifecycle token for a correlation key and registry generation.
    #[must_use]
    pub(super) const fn new(key: K, generation: u64) -> Self {
        Self { key, generation }
    }

    /// Return the response-correlation key carried by this token.
    #[must_use]
    pub const fn key(self) -> K {
        self.key
    }

    /// Return the registry generation carried by this token.
    #[must_use]
    pub(super) const fn generation(self) -> u64 {
        self.generation
    }
}

/// Cancellation handle retained by a deferred protocol response.
///
/// Dropping an armed handle notifies the owning protocol actor so it can remove
/// the corresponding correlation entry. The callback captures its typed token, so callers
/// awaiting a response do not need to carry the actor's key type.
pub struct Cancellation {
    cancel: Option<Box<dyn FnOnce() + Send>>,
}

impl Cancellation {
    /// Create an armed cancellation handle for a lifecycle token.
    pub(crate) fn new<K, F>(token: Token<K>, cancel: F) -> Self
    where
        K: Copy + Send + 'static,
        F: FnOnce(Token<K>) + Send + 'static,
    {
        Self {
            cancel: Some(Box::new(move || cancel(token))),
        }
    }

    /// Create an armed cancellation handle with the initial test generation.
    #[cfg(test)]
    pub fn test_new<K, F>(key: K, cancel: F) -> Self
    where
        K: Copy + Send + 'static,
        F: FnOnce(Token<K>) + Send + 'static,
    {
        Self::new(Token::new(key, TEST_GENERATION), cancel)
    }

    /// Prevent this handle from cancelling a correlation that has completed.
    pub fn disarm(&mut self) {
        self.cancel = None;
    }
}

impl Debug for Cancellation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Cancellation")
            .field("armed", &self.cancel.is_some())
            .finish()
    }
}

impl Drop for Cancellation {
    fn drop(&mut self) {
        if let Some(cancel) = self.cancel.take() {
            cancel();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::{Cancellation, TEST_GENERATION, Token};

    #[test]
    fn dropping_a_response_requests_actor_owned_cancellation() {
        let cancelled = Arc::new(Mutex::new(None));
        let cancellation_result = cancelled.clone();
        let expected = u8::MIN;
        let token = Token::new(expected, TEST_GENERATION);
        let cancellation = Cancellation::new(token, move |token| {
            *cancellation_result
                .lock()
                .expect("cancellation result lock remains available") = Some(token);
        });

        drop(cancellation);

        assert_eq!(
            *cancelled
                .lock()
                .expect("cancellation result lock remains available"),
            Some(token)
        );
    }

    #[test]
    fn disarming_a_response_does_not_request_cancellation() {
        let mut cancellation = Cancellation::test_new(u8::MIN, |_| {
            panic!("completed responses must not cancel their transaction");
        });

        cancellation.disarm();
        drop(cancellation);
    }
}
