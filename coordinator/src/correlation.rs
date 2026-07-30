use std::collections::BTreeMap;
use std::time::Duration;

use tokio::sync::oneshot::{Receiver, Sender, channel};

pub use self::key::Key;
use crate::Error;

mod key;

/// Maximum time retained for a pending ZCL or ZDP response.
pub const PROTOCOL_RESPONSE_TIMEOUT: Duration = Duration::from_secs(30);
/// Maximum additional time retained for a late ZCL or ZDP response.
pub const PROTOCOL_QUARANTINE_TIMEOUT: Duration = Duration::from_secs(30);
const INITIAL_GENERATION: u64 = 0;
const INITIAL_SEQUENCE: u8 = 0;
const TRANSACTION_SEQUENCE_COUNT: usize = 1_usize << u8::BITS;

type RegisteredResponse<T> = (u8, Token, Receiver<Result<T, Error>>);

/// Coordinator-private identity for lifecycle messages associated with one protocol transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Token {
    key: Key,
    generation: u64,
}

impl Token {
    /// Return the response-correlation key carried by this token.
    #[must_use]
    pub const fn key(self) -> Key {
        self.key
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
    pub(crate) fn new<F>(token: Token, cancel: F) -> Self
    where
        F: FnOnce(Token) + Send + 'static,
    {
        Self {
            token,
            cancel: Some(Box::new(cancel)),
        }
    }

    #[cfg(test)]
    pub fn test_new<F>(key: Key, cancel: F) -> Self
    where
        F: FnOnce(Token) + Send + 'static,
    {
        Self::new(
            Token {
                key,
                generation: INITIAL_GENERATION,
            },
            cancel,
        )
    }

    /// Prevent this handle from cancelling a correlation that has completed.
    pub fn disarm(&mut self) {
        self.cancel = None;
    }
}

impl std::fmt::Debug for Cancellation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

#[derive(Debug)]
struct Pending<T> {
    generation: u64,
    response: Sender<Result<T, Error>>,
}

/// Actor-owned protocol-response correlations and transaction sequences.
#[derive(Debug)]
pub struct Registry<T> {
    next_sequence: u8,
    next_generation: u64,
    pending: BTreeMap<Key, Pending<T>>,
    quarantined: BTreeMap<Key, u64>,
}

impl<T> Registry<T> {
    /// Create an empty response registry.
    pub const fn new() -> Self {
        Self {
            next_sequence: INITIAL_SEQUENCE,
            next_generation: INITIAL_GENERATION,
            pending: BTreeMap::new(),
            quarantined: BTreeMap::new(),
        }
    }

    /// Allocate and register a response correlation.
    pub fn register<F>(&mut self, key_for_sequence: F) -> Result<RegisteredResponse<T>, Error>
    where
        F: Fn(u8) -> Key,
    {
        let (sequence, key) = self.allocate(&key_for_sequence)?;
        let token = Token {
            key,
            generation: self.next_generation,
        };
        self.next_generation = self.next_generation.wrapping_add(1);
        let (response, receiver) = channel();
        let pending = Pending {
            generation: token.generation,
            response,
        };

        let previous = self.pending.insert(key, pending);
        debug_assert!(previous.is_none());

        Ok((sequence, token, receiver))
    }

    /// Allocate a sequence for a frame that does not expect a correlated response.
    ///
    /// The allocator skips identities used by tracked or quarantined requests, but does not retain
    /// the selected identity after returning it.
    pub fn allocate_untracked_sequence<F>(&mut self, key_for_sequence: F) -> Result<u8, Error>
    where
        F: Fn(u8) -> Option<Key>,
    {
        for _ in 0..TRANSACTION_SEQUENCE_COUNT {
            let sequence = self.take_next_sequence();
            let Some(key) = key_for_sequence(sequence) else {
                return Ok(sequence);
            };
            if self.key_is_available(key) {
                return Ok(sequence);
            }
        }

        Err(Error::TransactionSequenceExhausted)
    }

    /// Complete a pending response and release its transaction identity.
    pub fn complete(&mut self, key: Key, value: T) -> bool {
        let Some(pending) = self.pending.remove(&key) else {
            return false;
        };

        pending.response.send(Ok(value)).unwrap_or_else(drop);
        true
    }

    /// Cancel a pending response and quarantine its identity for late-response handling.
    pub fn cancel(&mut self, token: Token) -> bool {
        self.remove_and_quarantine(token).is_some()
    }

    /// Discard a correlation whose request was never handed to the hardware.
    pub fn discard(&mut self, token: Token) {
        if self.pending_generation_matches(token) {
            self.pending.remove(&token.key);
        }
    }

    /// Consume a late response and release its quarantined transaction identity.
    pub fn release_quarantine(&mut self, key: Key) -> bool {
        self.quarantined.remove(&key).is_some()
    }

    /// Release a quarantined identity after its bounded late-response grace period.
    pub fn expire_quarantine(&mut self, token: Token) -> bool {
        let generation_matches = self
            .quarantined
            .get(&token.key)
            .is_some_and(|generation| *generation == token.generation);
        if generation_matches {
            self.quarantined.remove(&token.key);
        }
        generation_matches
    }

    /// Fail one pending response whose actor-owned timeout message arrived.
    pub fn timeout(&mut self, token: Token) -> bool {
        let Some(pending) = self.remove_and_quarantine(token) else {
            return false;
        };
        pending
            .response
            .send(Err(Error::ProtocolResponseTimeout))
            .unwrap_or_else(drop);
        true
    }

    /// Fail every pending response and start a fresh network correlation epoch.
    pub fn network_down(&mut self, error: &zb_hw::TransmissionError) {
        self.fail_all(|| zb_hw::Error::from(error.clone()).into());
    }

    /// Fail every pending response because the hardware event source is unavailable.
    pub fn hardware_unavailable(&mut self) {
        self.fail_all(|| zb_hw::Error::ActorUnavailable.into());
    }

    fn fail_all<F>(&mut self, mut error: F)
    where
        F: FnMut() -> Error,
    {
        let pending = std::mem::take(&mut self.pending);
        self.quarantined.clear();
        self.next_sequence = INITIAL_SEQUENCE;

        for pending in pending.into_values() {
            pending.response.send(Err(error())).unwrap_or_else(drop);
        }
    }

    fn allocate<F>(&mut self, key_for_sequence: &F) -> Result<(u8, Key), Error>
    where
        F: Fn(u8) -> Key,
    {
        for _ in 0..TRANSACTION_SEQUENCE_COUNT {
            let sequence = self.take_next_sequence();
            let key = key_for_sequence(sequence);
            if self.key_is_available(key) {
                return Ok((sequence, key));
            }
        }

        Err(Error::TransactionSequenceExhausted)
    }

    const fn take_next_sequence(&mut self) -> u8 {
        let sequence = self.next_sequence;
        self.next_sequence = self.next_sequence.wrapping_add(1);
        sequence
    }

    fn key_is_available(&self, key: Key) -> bool {
        !self.pending.contains_key(&key) && !self.quarantined.contains_key(&key)
    }

    fn remove_and_quarantine(&mut self, token: Token) -> Option<Pending<T>> {
        if !self.pending_generation_matches(token) {
            return None;
        }

        let pending = self.pending.remove(&token.key);
        let newly_quarantined = self
            .quarantined
            .insert(token.key, token.generation)
            .is_none();
        debug_assert!(newly_quarantined);
        pending
    }

    fn pending_generation_matches(&self, token: Token) -> bool {
        self.pending
            .get(&token.key)
            .is_some_and(|pending| pending.generation == token.generation)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::{
        Cancellation, INITIAL_GENERATION, Key, Registry, TRANSACTION_SEQUENCE_COUNT, Token,
    };
    use crate::Error;

    const CLUSTER_ID: u16 = 1;
    const OTHER_SHORT_ID: u16 = 4;
    const PROFILE_ID: u16 = 2;
    const SHORT_ID: u16 = 3;

    #[test]
    fn refuses_to_replace_any_pending_sequence() {
        let mut registry = Registry::<()>::new();
        let mut responses = Vec::new();

        for _ in 0..TRANSACTION_SEQUENCE_COUNT {
            let (_, _, response) = registry
                .register(key)
                .expect("all transaction sequences are initially available");
            responses.push(response);
        }

        assert!(matches!(
            registry.register(key),
            Err(Error::TransactionSequenceExhausted)
        ));
        assert_eq!(responses.len(), TRANSACTION_SEQUENCE_COUNT);
    }

    #[test]
    fn dropping_a_response_requests_actor_owned_cancellation() {
        let cancelled = Arc::new(Mutex::new(None));
        let cancellation_result = cancelled.clone();
        let expected = key(u8::MIN);
        let token = Token {
            key: expected,
            generation: INITIAL_GENERATION,
        };
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
    fn completed_sequence_is_immediately_reallocated() {
        let mut registry = Registry::<()>::new();
        let (first_sequence, first_token, _response) = registry
            .register(key)
            .expect("transaction sequence is available");
        assert!(registry.complete(first_token.key(), ()));
        registry.next_sequence = first_sequence;

        let (second_sequence, _, _response) = registry
            .register(key)
            .expect("another transaction sequence is available");

        assert_eq!(second_sequence, first_sequence);
    }

    #[test]
    fn actor_expiration_returns_a_protocol_timeout() {
        let mut registry = Registry::<()>::new();
        let (sequence, token, response) = registry
            .register(key)
            .expect("transaction sequence is available");

        registry.timeout(token);

        assert!(matches!(
            response.blocking_recv(),
            Ok(Err(Error::ProtocolResponseTimeout))
        ));
        assert!(registry.quarantined.contains_key(&key(sequence)));
        assert!(registry.release_quarantine(key(sequence)));
        assert!(!registry.quarantined.contains_key(&key(sequence)));
    }

    #[test]
    fn untracked_sequences_wrap_without_exhaustion() {
        let mut registry = Registry::<()>::new();
        let mut sequences = Vec::new();

        for _ in 0..TRANSACTION_SEQUENCE_COUNT * 2 {
            sequences.push(
                registry
                    .allocate_untracked_sequence(|sequence| Some(key(sequence)))
                    .expect("untracked transaction sequences remain available"),
            );
        }

        assert_eq!(sequences[0], sequences[TRANSACTION_SEQUENCE_COUNT]);
        assert!(registry.quarantined.is_empty());
    }

    #[test]
    fn untracked_allocation_skips_a_pending_identity() {
        let mut registry = Registry::<()>::new();
        let (pending_sequence, _token, _response) = registry
            .register(key)
            .expect("transaction sequence is available");
        registry.next_sequence = pending_sequence;

        let untracked_sequence = registry
            .allocate_untracked_sequence(|sequence| Some(key(sequence)))
            .expect("another transaction sequence is available");

        assert_ne!(untracked_sequence, pending_sequence);
    }

    #[test]
    fn network_boundary_releases_quarantined_identities() {
        let mut registry = Registry::<()>::new();
        let mut responses = Vec::new();
        for _ in 0..TRANSACTION_SEQUENCE_COUNT {
            let (_, token, response) = registry
                .register(key)
                .expect("all transaction sequences are initially available");
            assert!(registry.cancel(token));
            responses.push(response);
        }
        assert!(matches!(
            registry.register(key),
            Err(Error::TransactionSequenceExhausted)
        ));

        registry.network_down(&zb_hw::TransmissionError::NoRoute);

        assert_eq!(
            registry
                .allocate_untracked_sequence(|sequence| Some(key(sequence)))
                .expect("network boundary starts a fresh correlation epoch"),
            u8::MIN
        );
        assert_eq!(responses.len(), TRANSACTION_SEQUENCE_COUNT);
    }

    #[test]
    fn network_failure_resolves_every_pending_response() {
        let mut registry = Registry::<()>::new();
        let (_, first_token, first_response) = registry
            .register(key)
            .expect("transaction sequence is available");
        let (_, _, second_response) = registry
            .register(key)
            .expect("transaction sequence is available");
        registry.cancel(first_token);

        registry.network_down(&zb_hw::TransmissionError::NoRoute);

        assert!(first_response.blocking_recv().is_err());
        assert!(matches!(
            second_response.blocking_recv(),
            Ok(Err(Error::Hardware(zb_hw::Error::Transmission(
                zb_hw::TransmissionError::NoRoute
            ))))
        ));
        assert!(registry.quarantined.is_empty());
    }

    #[test]
    fn hardware_unavailability_resolves_every_pending_response() {
        let mut registry = Registry::<()>::new();
        let (_, first_token, first_response) = registry
            .register(key)
            .expect("transaction sequence is available");
        let (_, _, second_response) = registry
            .register(key)
            .expect("transaction sequence is available");
        registry.cancel(first_token);

        registry.hardware_unavailable();

        assert!(first_response.blocking_recv().is_err());
        assert!(matches!(
            second_response.blocking_recv(),
            Ok(Err(Error::Hardware(zb_hw::Error::ActorUnavailable)))
        ));
        assert!(registry.quarantined.is_empty());
    }

    #[test]
    fn stale_lifecycle_token_does_not_remove_reused_transaction() {
        let mut registry = Registry::<()>::new();
        let (sequence, stale_token, stale_response) = registry
            .register(key)
            .expect("transaction sequence is available");
        assert!(registry.complete(stale_token.key(), ()));
        assert!(matches!(stale_response.blocking_recv(), Ok(Ok(()))));
        registry.next_sequence = sequence;
        let (_, current_token, mut current_response) = registry
            .register(key)
            .expect("transaction sequence can be reused after completion");

        registry.cancel(stale_token);
        registry.timeout(stale_token);

        assert!(registry.pending.contains_key(&current_token.key()));
        assert!(current_response.try_recv().is_err());
    }

    #[test]
    fn stale_quarantine_timeout_does_not_release_a_newer_generation() {
        let mut registry = Registry::<()>::new();
        let (sequence, stale_token, _response) = registry
            .register(key)
            .expect("transaction sequence is available");
        assert!(registry.cancel(stale_token));
        assert!(registry.expire_quarantine(stale_token));

        registry.next_sequence = sequence;
        let (_, current_token, _response) = registry
            .register(key)
            .expect("transaction sequence can be reused after quarantine expiry");
        assert!(registry.cancel(current_token));

        assert!(!registry.expire_quarantine(stale_token));
        assert!(registry.quarantined.contains_key(&current_token.key()));
        assert!(registry.expire_quarantine(current_token));
    }

    #[test]
    fn discarded_unsent_transaction_is_not_quarantined() {
        let mut registry = Registry::<()>::new();
        let (sequence, token, response) = registry
            .register(key)
            .expect("transaction sequence is available");

        registry.discard(token);
        registry.next_sequence = sequence;

        assert!(response.blocking_recv().is_err());
        assert_eq!(
            registry
                .register(key)
                .expect("unsent transaction identity is immediately reusable")
                .0,
            sequence
        );
    }

    #[test]
    fn quarantine_is_scoped_to_the_complete_correlation_key() {
        let mut registry = Registry::<()>::new();
        let (sequence, token, _response) = registry
            .register(key)
            .expect("transaction sequence is available");
        registry.cancel(token);
        registry.next_sequence = sequence;

        let (reused_sequence, _, _response) = registry
            .register(|sequence| {
                Key::new(
                    OTHER_SHORT_ID,
                    zb_core::Endpoint::Data,
                    CLUSTER_ID,
                    PROFILE_ID,
                    None,
                    sequence,
                )
            })
            .expect("another correlation domain can reuse the sequence");

        assert_eq!(reused_sequence, sequence);
    }

    fn key(sequence: u8) -> Key {
        Key::new(
            SHORT_ID,
            zb_core::Endpoint::Data,
            CLUSTER_ID,
            PROFILE_ID,
            None,
            sequence,
        )
    }
}
