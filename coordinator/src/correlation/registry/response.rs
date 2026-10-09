use bytes::Bytes;
use tokio::sync::oneshot::{Sender, channel};

use super::{Key, RegisteredResponse, Registry};
use crate::Error;

/// One response sink per reserved identity; raw delivery never needs a forwarding task.
#[derive(Debug)]
pub(super) enum Response<T> {
    Typed(Sender<Result<T, Error>>),
    Raw(Sender<Result<Bytes, Error>>),
}

impl<T> Response<T> {
    pub(super) fn fail(self, error: Error) {
        match self {
            Self::Typed(response) => response.send(Err(error)).unwrap_or_else(drop),
            Self::Raw(response) => response.send(Err(error)).unwrap_or_else(drop),
        }
    }
}

#[derive(Debug)]
pub(super) struct Pending<T, Matcher> {
    pub(super) generation: u64,
    pub(super) response: Response<T>,
    pub(super) matcher: Matcher,
}

/// Retain matching policy after cancellation/timeout so unrelated frames cannot release a TSN.
#[derive(Debug)]
pub(super) struct Quarantined<Matcher> {
    pub(super) generation: u64,
    pub(super) matcher: Matcher,
}

impl<T, Matcher> Registry<T, Matcher> {
    /// Register a typed sink with matching policy retained through quarantine.
    pub fn try_register_matching<F>(
        &mut self,
        key_for_sequence: F,
        matcher: Matcher,
    ) -> Result<RegisteredResponse<T>, Error>
    where
        F: Fn(u8) -> Result<Key, Error>,
    {
        let (response, receiver) = channel();
        let (sequence, token) =
            self.insert(key_for_sequence, Response::Typed(response), matcher)?;
        Ok((sequence, token, receiver))
    }

    /// Register a raw sink in the same allocator and pending registry as typed requests.
    pub fn try_register_raw<F>(
        &mut self,
        key_for_sequence: F,
        matcher: Matcher,
    ) -> Result<RegisteredResponse<Bytes>, Error>
    where
        F: Fn(u8) -> Result<Key, Error>,
    {
        let (response, receiver) = channel();
        let (sequence, token) = self.insert(key_for_sequence, Response::Raw(response), matcher)?;
        Ok((sequence, token, receiver))
    }

    /// Match before removal: a raw waiter must not consume a typed response through fallthrough.
    pub fn complete_matching<F>(&mut self, key: Key, value: T, matches: F) -> bool
    where
        F: FnOnce(&Matcher) -> bool,
    {
        if !self.pending.get(&key).is_some_and(|pending| {
            matches(&pending.matcher) && matches!(pending.response, Response::Typed(_))
        }) {
            return false;
        }
        let Some(Pending {
            response: Response::Typed(response),
            ..
        }) = self.pending.remove(&key)
        else {
            return false;
        };
        response.send(Ok(value)).unwrap_or_else(drop);
        true
    }

    /// Deliver the original ASDU only to a matching raw sink.
    pub fn complete_raw<F>(&mut self, key: Key, asdu: Bytes, matches: F) -> bool
    where
        F: FnOnce(&Matcher) -> bool,
    {
        if !self.pending.get(&key).is_some_and(|pending| {
            matches(&pending.matcher) && matches!(pending.response, Response::Raw(_))
        }) {
            return false;
        }
        let Some(Pending {
            response: Response::Raw(response),
            ..
        }) = self.pending.remove(&key)
        else {
            return false;
        };
        response.send(Ok(asdu)).unwrap_or_else(drop);
        true
    }

    /// Release quarantine only after checking its retained response matching policy.
    pub fn release_quarantine_matching<F>(&mut self, key: Key, matches: F) -> bool
    where
        F: FnOnce(&Matcher) -> bool,
    {
        if !self
            .quarantined
            .get(&key)
            .is_some_and(|entry| matches(&entry.matcher))
        {
            return false;
        }
        self.quarantined.remove(&key).is_some()
    }
}
