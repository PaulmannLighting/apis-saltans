use std::fmt::{Debug, Formatter};

use super::Key;

#[cfg(test)]
const TEST_GENERATION: u64 = 0;

/// Coordinator-private identity for lifecycle messages associated with one protocol transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Token {
    key: Key,
    generation: u64,
}

impl Token {
    /// Create a lifecycle token for a correlation key and registry generation.
    #[must_use]
    pub(super) const fn new(key: Key, generation: u64) -> Self {
        Self { key, generation }
    }

    /// Return the response-correlation key carried by this token.
    #[must_use]
    pub const fn key(self) -> Key {
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
/// the corresponding correlation entry.
pub struct Cancellation {
    token: Token,
    cancel: Option<Box<dyn FnOnce(Token) + Send>>,
}

impl Cancellation {
    /// Create an armed cancellation handle for a lifecycle token.
    pub(crate) fn new<F>(token: Token, cancel: F) -> Self
    where
        F: FnOnce(Token) + Send + 'static,
    {
        Self {
            token,
            cancel: Some(Box::new(cancel)),
        }
    }

    /// Create an armed cancellation handle with the initial test generation.
    #[cfg(test)]
    pub fn test_new<F>(key: Key, cancel: F) -> Self
    where
        F: FnOnce(Token) + Send + 'static,
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
            .field("token", &self.token)
            .field("armed", &self.cancel.is_some())
            .finish()
    }
}

impl Drop for Cancellation {
    fn drop(&mut self) {
        if let Some(cancel) = self.cancel.take() {
            cancel(self.token);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use zb_core::Endpoint;

    use super::{Cancellation, TEST_GENERATION, Token};
    use crate::correlation::Key;

    const CLUSTER_ID: u16 = 1;
    const PROFILE_ID: u16 = 2;
    const SHORT_ID: u16 = 3;

    #[test]
    fn dropping_a_response_requests_actor_owned_cancellation() {
        let cancelled = Arc::new(Mutex::new(None));
        let cancellation_result = cancelled.clone();
        let expected = key(u8::MIN);
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

    fn key(sequence: u8) -> Key {
        Key::new(
            SHORT_ID,
            Endpoint::Data,
            CLUSTER_ID,
            PROFILE_ID,
            None,
            sequence,
        )
    }
}
