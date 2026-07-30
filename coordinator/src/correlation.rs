use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;

use tokio::sync::oneshot::{Receiver, Sender, channel};

use crate::Error;
use crate::index::Index;

/// Maximum time retained for a pending ZCL or ZDP response.
pub const PROTOCOL_RESPONSE_TIMEOUT: Duration = Duration::from_secs(30);
const SEQUENCE_REUSE_DELAY: Duration = Duration::from_secs(2);
const TRANSACTION_SEQUENCE_COUNT: usize = 1_usize << u8::BITS;

type RegisteredResponse<T> = (u8, Index, Receiver<Result<T, Error>>);

/// Cancellation handle retained by a deferred protocol response.
///
/// Dropping an armed handle notifies the owning protocol actor so it can remove
/// the corresponding correlation entry.
pub struct Cancellation {
    index: Index,
    cancel: Option<Box<dyn FnOnce(Index) + Send>>,
}

impl Cancellation {
    pub(crate) fn new<F>(index: Index, cancel: F) -> Self
    where
        F: FnOnce(Index) + Send + 'static,
    {
        Self {
            index,
            cancel: Some(Box::new(cancel)),
        }
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
            .field("index", &self.index)
            .field("armed", &self.cancel.is_some())
            .finish()
    }
}

impl Drop for Cancellation {
    fn drop(&mut self) {
        if let Some(cancel) = self.cancel.take() {
            cancel(self.index);
        }
    }
}

#[derive(Debug)]
struct Pending<T> {
    response: Sender<Result<T, Error>>,
}

/// Actor-owned protocol-response correlations and transaction sequences.
#[derive(Debug)]
pub struct Registry<T> {
    next_sequence: u8,
    pending: BTreeMap<Index, Pending<T>>,
    occupied_sequences: BTreeSet<u8>,
    quarantined: BTreeMap<Index, tokio::time::Instant>,
}

impl<T> Registry<T> {
    /// Create an empty response registry.
    pub const fn new() -> Self {
        Self {
            next_sequence: 0,
            pending: BTreeMap::new(),
            occupied_sequences: BTreeSet::new(),
            quarantined: BTreeMap::new(),
        }
    }

    /// Allocate and register a response correlation.
    pub fn register<F>(&mut self, index_for_sequence: F) -> Result<RegisteredResponse<T>, Error>
    where
        F: Fn(u8) -> Index,
    {
        let sequence = self.allocate_sequence()?;
        let index = index_for_sequence(sequence);
        let (response, receiver) = channel();
        let pending = Pending { response };

        let previous = self.pending.insert(index, pending);
        debug_assert!(previous.is_none());
        let newly_occupied = self.occupied_sequences.insert(sequence);
        debug_assert!(newly_occupied);

        Ok((sequence, index, receiver))
    }

    /// Reserve a sequence for a frame that does not expect a correlated response.
    pub fn reserve_untracked_sequence<F>(&mut self, index_for_sequence: F) -> Result<u8, Error>
    where
        F: Fn(u8) -> Option<Index>,
    {
        let sequence = self.allocate_sequence()?;
        if let Some(index) = index_for_sequence(sequence) {
            self.quarantine(index);
        }
        Ok(sequence)
    }

    /// Complete a pending response and quarantine its sequence against late frames.
    pub fn complete(&mut self, index: Index, value: T) -> bool {
        let Some(pending) = self.remove(index) else {
            return false;
        };

        pending.response.send(Ok(value)).unwrap_or_else(drop);
        true
    }

    /// Cancel a pending response and quarantine its sequence against late frames.
    pub fn cancel(&mut self, index: Index) {
        self.remove(index);
    }

    /// Return whether a received transaction sequence is still quarantined.
    pub fn is_quarantined(&mut self, index: Index) -> bool {
        self.remove_expired_quarantines();
        self.quarantined.contains_key(&index)
    }

    /// Fail one pending response whose actor-owned timeout message arrived.
    pub fn timeout(&mut self, index: Index) {
        let Some(pending) = self.remove(index) else {
            return;
        };
        pending
            .response
            .send(Err(Error::ProtocolResponseTimeout))
            .unwrap_or_else(drop);
    }

    /// Fail and release every pending response.
    pub fn fail_all(&mut self, error: &zb_hw::TransmissionError) {
        let indexes: Vec<_> = self.pending.keys().copied().collect();

        for index in indexes {
            let Some(pending) = self.remove(index) else {
                continue;
            };
            pending
                .response
                .send(Err(zb_hw::Error::from(error.clone()).into()))
                .unwrap_or_else(drop);
        }
    }

    fn allocate_sequence(&mut self) -> Result<u8, Error> {
        self.remove_expired_quarantines();

        for _ in 0..TRANSACTION_SEQUENCE_COUNT {
            let sequence = self.next_sequence;
            self.next_sequence = self.next_sequence.wrapping_add(1);

            if !self.occupied_sequences.contains(&sequence)
                && !self
                    .quarantined
                    .keys()
                    .any(|index| index.sequence() == sequence)
            {
                return Ok(sequence);
            }
        }

        Err(Error::TransactionSequenceExhausted)
    }

    fn remove(&mut self, index: Index) -> Option<Pending<T>> {
        let pending = self.pending.remove(&index)?;
        let sequence = index.sequence();
        let was_occupied = self.occupied_sequences.remove(&sequence);
        debug_assert!(was_occupied);
        self.quarantine(index);
        Some(pending)
    }

    fn quarantine(&mut self, index: Index) {
        self.quarantined
            .insert(index, tokio::time::Instant::now() + SEQUENCE_REUSE_DELAY);
    }

    fn remove_expired_quarantines(&mut self) {
        let now = tokio::time::Instant::now();
        self.quarantined.retain(|_, deadline| *deadline > now);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::{Cancellation, Registry, TRANSACTION_SEQUENCE_COUNT};
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
        let cancellation = Cancellation::new(expected, move |index| {
            *cancellation_result
                .lock()
                .expect("cancellation result lock remains available") = Some(index);
        });

        drop(cancellation);

        assert_eq!(
            *cancelled
                .lock()
                .expect("cancellation result lock remains available"),
            Some(expected)
        );
    }

    #[test]
    fn completed_sequence_is_not_immediately_reallocated() {
        let mut registry = Registry::<()>::new();
        let (first_sequence, first_index, _response) = registry
            .register(index)
            .expect("transaction sequence is available");
        assert!(registry.complete(first_index, ()));
        registry.next_sequence = first_sequence;

        let (second_sequence, _, _response) = registry
            .register(index)
            .expect("another transaction sequence is available");

        assert_ne!(second_sequence, first_sequence);
        assert!(registry.is_quarantined(index(first_sequence)));
        assert!(!registry.is_quarantined(Index::new(
            OTHER_SHORT_ID,
            zb_core::Endpoint::Data,
            CLUSTER_ID,
            PROFILE_ID,
            None,
            first_sequence,
        )));
    }

    #[test]
    fn actor_expiration_returns_a_protocol_timeout() {
        let mut registry = Registry::<()>::new();
        let (sequence, response_index, response) = registry
            .register(index)
            .expect("transaction sequence is available");

        registry.timeout(response_index);

        assert!(matches!(
            response.blocking_recv(),
            Ok(Err(Error::ProtocolResponseTimeout))
        ));
        assert!(registry.is_quarantined(index(sequence)));
    }

    #[test]
    fn network_failure_resolves_every_pending_response() {
        let mut registry = Registry::<()>::new();
        let (first_sequence, _, first_response) = registry
            .register(index)
            .expect("transaction sequence is available");
        let (_, _, second_response) = registry
            .register(index)
            .expect("transaction sequence is available");

        registry.fail_all(&zb_hw::TransmissionError::NoRoute);

        for response in [first_response, second_response] {
            assert!(matches!(
                response.blocking_recv(),
                Ok(Err(Error::Hardware(zb_hw::Error::Transmission(
                    zb_hw::TransmissionError::NoRoute
                ))))
            ));
        }
        assert!(registry.is_quarantined(index(first_sequence)));
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
