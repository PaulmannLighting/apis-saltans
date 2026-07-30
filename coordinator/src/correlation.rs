use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;

use tokio::sync::oneshot::{Receiver, Sender, channel};

use crate::Error;
use crate::index::Index;

/// Maximum time retained for a pending ZCL or ZDP response.
pub const PROTOCOL_RESPONSE_TIMEOUT: Duration = Duration::from_secs(30);
const INITIAL_GENERATION: u64 = 0;
const INITIAL_SEQUENCE: u8 = 0;
const TRANSACTION_SEQUENCE_COUNT: usize = 1_usize << u8::BITS;

type RegisteredResponse<T> = (u8, Token, Receiver<Result<T, Error>>);

/// Coordinator-private identity for lifecycle messages associated with one protocol transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Token {
    index: Index,
    generation: u64,
}

impl Token {
    /// Return the response-correlation index carried by this token.
    #[must_use]
    pub const fn index(self) -> Index {
        self.index
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
    pub fn test_new<F>(index: Index, cancel: F) -> Self
    where
        F: FnOnce(Token) + Send + 'static,
    {
        Self::new(
            Token {
                index,
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
    pending: BTreeMap<Index, Pending<T>>,
    quarantined: BTreeSet<Index>,
}

impl<T> Registry<T> {
    /// Create an empty response registry.
    pub const fn new() -> Self {
        Self {
            next_sequence: INITIAL_SEQUENCE,
            next_generation: INITIAL_GENERATION,
            pending: BTreeMap::new(),
            quarantined: BTreeSet::new(),
        }
    }

    /// Allocate and register a response correlation.
    pub fn register<F>(&mut self, index_for_sequence: F) -> Result<RegisteredResponse<T>, Error>
    where
        F: Fn(u8) -> Index,
    {
        let (sequence, index) = self.allocate(&index_for_sequence)?;
        let token = Token {
            index,
            generation: self.next_generation,
        };
        self.next_generation = self.next_generation.wrapping_add(1);
        let (response, receiver) = channel();
        let pending = Pending {
            generation: token.generation,
            response,
        };

        let previous = self.pending.insert(index, pending);
        debug_assert!(previous.is_none());

        Ok((sequence, token, receiver))
    }

    /// Reserve a sequence for a frame that does not expect a correlated response.
    pub fn reserve_untracked_sequence<F>(&mut self, index_for_sequence: F) -> Result<u8, Error>
    where
        F: Fn(u8) -> Option<Index>,
    {
        for _ in 0..TRANSACTION_SEQUENCE_COUNT {
            let sequence = self.take_next_sequence();
            let Some(index) = index_for_sequence(sequence) else {
                return Ok(sequence);
            };
            if self.index_is_available(index) {
                self.quarantined.insert(index);
                return Ok(sequence);
            }
        }

        Err(Error::TransactionSequenceExhausted)
    }

    /// Complete a pending response and release its transaction identity.
    pub fn complete(&mut self, index: Index, value: T) -> bool {
        let Some(pending) = self.pending.remove(&index) else {
            return false;
        };

        pending.response.send(Ok(value)).unwrap_or_else(drop);
        true
    }

    /// Cancel a pending response and quarantine its identity until a late frame arrives.
    pub fn cancel(&mut self, token: Token) {
        self.remove_and_quarantine(token);
    }

    /// Discard a correlation whose request was never handed to the hardware.
    pub fn discard(&mut self, token: Token) {
        if self.pending_generation_matches(token) {
            self.pending.remove(&token.index);
        }
    }

    /// Consume a late response and release its quarantined transaction identity.
    pub fn release_quarantine(&mut self, index: Index) -> bool {
        self.quarantined.remove(&index)
    }

    /// Fail one pending response whose actor-owned timeout message arrived.
    pub fn timeout(&mut self, token: Token) {
        let Some(pending) = self.remove_and_quarantine(token) else {
            return;
        };
        pending
            .response
            .send(Err(Error::ProtocolResponseTimeout))
            .unwrap_or_else(drop);
    }

    /// Fail every pending response and start a fresh network correlation epoch.
    pub fn network_down(&mut self, error: &zb_hw::TransmissionError) {
        let pending = std::mem::take(&mut self.pending);
        self.quarantined.clear();
        self.next_sequence = INITIAL_SEQUENCE;

        for pending in pending.into_values() {
            pending
                .response
                .send(Err(zb_hw::Error::from(error.clone()).into()))
                .unwrap_or_else(drop);
        }
    }

    fn allocate<F>(&mut self, index_for_sequence: &F) -> Result<(u8, Index), Error>
    where
        F: Fn(u8) -> Index,
    {
        for _ in 0..TRANSACTION_SEQUENCE_COUNT {
            let sequence = self.take_next_sequence();
            let index = index_for_sequence(sequence);
            if self.index_is_available(index) {
                return Ok((sequence, index));
            }
        }

        Err(Error::TransactionSequenceExhausted)
    }

    const fn take_next_sequence(&mut self) -> u8 {
        let sequence = self.next_sequence;
        self.next_sequence = self.next_sequence.wrapping_add(1);
        sequence
    }

    fn index_is_available(&self, index: Index) -> bool {
        !self.pending.contains_key(&index) && !self.quarantined.contains(&index)
    }

    fn remove_and_quarantine(&mut self, token: Token) -> Option<Pending<T>> {
        if !self.pending_generation_matches(token) {
            return None;
        }

        let pending = self.pending.remove(&token.index);
        let newly_quarantined = self.quarantined.insert(token.index);
        debug_assert!(newly_quarantined);
        pending
    }

    fn pending_generation_matches(&self, token: Token) -> bool {
        self.pending
            .get(&token.index)
            .is_some_and(|pending| pending.generation == token.generation)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::{Cancellation, INITIAL_GENERATION, Registry, TRANSACTION_SEQUENCE_COUNT, Token};
    use crate::Error;
    use crate::index::Index;

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
                .register(index)
                .expect("all transaction sequences are initially available");
            responses.push(response);
        }

        assert!(matches!(
            registry.register(index),
            Err(Error::TransactionSequenceExhausted)
        ));
        assert_eq!(responses.len(), TRANSACTION_SEQUENCE_COUNT);
    }

    #[test]
    fn dropping_a_response_requests_actor_owned_cancellation() {
        let cancelled = Arc::new(Mutex::new(None));
        let cancellation_result = cancelled.clone();
        let expected = index(u8::MIN);
        let token = Token {
            index: expected,
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
            .register(index)
            .expect("transaction sequence is available");
        assert!(registry.complete(first_token.index(), ()));
        registry.next_sequence = first_sequence;

        let (second_sequence, _, _response) = registry
            .register(index)
            .expect("another transaction sequence is available");

        assert_eq!(second_sequence, first_sequence);
    }

    #[test]
    fn actor_expiration_returns_a_protocol_timeout() {
        let mut registry = Registry::<()>::new();
        let (sequence, token, response) = registry
            .register(index)
            .expect("transaction sequence is available");

        registry.timeout(token);

        assert!(matches!(
            response.blocking_recv(),
            Ok(Err(Error::ProtocolResponseTimeout))
        ));
        assert!(registry.quarantined.contains(&index(sequence)));
        assert!(registry.release_quarantine(index(sequence)));
        assert!(!registry.quarantined.contains(&index(sequence)));
    }

    #[test]
    fn quarantined_identity_is_unavailable_until_its_late_frame_arrives() {
        let mut registry = Registry::<()>::new();
        let first_sequence = registry
            .reserve_untracked_sequence(|sequence| Some(index(sequence)))
            .expect("transaction sequence is available");
        registry.next_sequence = first_sequence;

        let second_sequence = registry
            .reserve_untracked_sequence(|sequence| Some(index(sequence)))
            .expect("another transaction sequence is available");

        assert_ne!(second_sequence, first_sequence);
        assert!(registry.release_quarantine(index(first_sequence)));
        registry.next_sequence = first_sequence;
        assert_eq!(
            registry
                .reserve_untracked_sequence(|sequence| Some(index(sequence)))
                .expect("late response released the transaction identity"),
            first_sequence
        );
    }

    #[test]
    fn network_boundary_releases_quarantined_identities() {
        let mut registry = Registry::<()>::new();
        for _ in 0..TRANSACTION_SEQUENCE_COUNT {
            registry
                .reserve_untracked_sequence(|sequence| Some(index(sequence)))
                .expect("all transaction sequences are initially available");
        }
        assert!(matches!(
            registry.reserve_untracked_sequence(|sequence| Some(index(sequence))),
            Err(Error::TransactionSequenceExhausted)
        ));

        registry.network_down(&zb_hw::TransmissionError::NoRoute);

        assert_eq!(
            registry
                .reserve_untracked_sequence(|sequence| Some(index(sequence)))
                .expect("network boundary starts a fresh correlation epoch"),
            u8::MIN
        );
    }

    #[test]
    fn network_failure_resolves_every_pending_response() {
        let mut registry = Registry::<()>::new();
        let (_, first_token, first_response) = registry
            .register(index)
            .expect("transaction sequence is available");
        let (_, _, second_response) = registry
            .register(index)
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
    fn stale_lifecycle_token_does_not_remove_reused_transaction() {
        let mut registry = Registry::<()>::new();
        let (sequence, stale_token, stale_response) = registry
            .register(index)
            .expect("transaction sequence is available");
        assert!(registry.complete(stale_token.index(), ()));
        assert!(matches!(stale_response.blocking_recv(), Ok(Ok(()))));
        registry.next_sequence = sequence;
        let (_, current_token, mut current_response) = registry
            .register(index)
            .expect("transaction sequence can be reused after completion");

        registry.cancel(stale_token);
        registry.timeout(stale_token);

        assert!(registry.pending.contains_key(&current_token.index()));
        assert!(current_response.try_recv().is_err());
    }

    #[test]
    fn discarded_unsent_transaction_is_not_quarantined() {
        let mut registry = Registry::<()>::new();
        let (sequence, token, response) = registry
            .register(index)
            .expect("transaction sequence is available");

        registry.discard(token);
        registry.next_sequence = sequence;

        assert!(response.blocking_recv().is_err());
        assert_eq!(
            registry
                .register(index)
                .expect("unsent transaction identity is immediately reusable")
                .0,
            sequence
        );
    }

    #[test]
    fn quarantine_is_scoped_to_the_complete_correlation_index() {
        let mut registry = Registry::<()>::new();
        let (sequence, token, _response) = registry
            .register(index)
            .expect("transaction sequence is available");
        registry.cancel(token);
        registry.next_sequence = sequence;

        let (reused_sequence, _, _response) = registry
            .register(|sequence| {
                Index::new(
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

    fn index(sequence: u8) -> Index {
        Index::new(
            SHORT_ID,
            zb_core::Endpoint::Data,
            CLUSTER_ID,
            PROFILE_ID,
            None,
            sequence,
        )
    }
}
