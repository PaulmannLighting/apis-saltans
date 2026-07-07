//! APS data-frame defragmentation.
//!
//! The assembler consumes NWK envelopes carrying raw APS data frames. It uses
//! the NWK source and APS counter as the transaction key, buffers payload
//! fragments, and returns the rebuilt APS data frame once all fragments are
//! present.

use std::collections::BTreeMap;
use std::num::NonZero;

use apis_saltans_nwk::Envelope;
use bytes::Bytes;
use log::{trace, warn};

use self::index::Index;
use self::transaction::{InsertResult, Transaction};
use crate::ExtendedControl;
use crate::data::Frame;

mod index;
mod transaction;

/// Reassembles fragmented APS data frames.
///
/// `Assembler` is stateful. Unfragmented frames are returned immediately. The
/// first fragment of a fragmented frame opens a transaction and follow-up
/// fragments are inserted into that transaction until all payload blocks are
/// present.
///
/// Completed frames are returned with their extended header removed, because the
/// returned payload is no longer fragmented.
#[derive(Debug, Default)]
pub struct Assembler {
    transactions: BTreeMap<Index, Transaction>,
}

impl Assembler {
    /// Add an APS data frame to the assembler.
    ///
    /// Returns `Some(frame)` for unfragmented frames and for fragmented frames
    /// whose final missing fragment completed a transaction. Returns `None`
    /// while a transaction is still incomplete or when the incoming frame is
    /// invalid and must be dropped.
    ///
    /// A transaction is identified by the envelope source and the APS frame
    /// counter. If a new first fragment arrives for an existing transaction, the
    /// previous transaction is dropped and replaced.
    #[must_use]
    pub fn add(&mut self, envelope: Envelope<Frame<Bytes>>) -> Option<Frame<Bytes>> {
        trace!("Received NWK envelope: {envelope:?}");

        let (source, _metadata, aps) = envelope.into_parts();

        let Some(extended) = aps.header().extended() else {
            trace!("APS frame has no extended header.");
            return Some(aps);
        };

        trace!("APS frame has extended header: {extended:?}");

        if extended
            .control()
            .contains(ExtendedControl::FIRST_FRAGMENT | ExtendedControl::FOLLOWUP_FRAGMENT)
        {
            warn!(
                "Dropping invalid frame that claims to be the first and a follow-up fragment of the transaction."
            );
            return None;
        }

        if extended.control().contains(ExtendedControl::FIRST_FRAGMENT) {
            trace!("APS frame is first fragment.");

            let Some(blocks) = extended.block_number() else {
                warn!("Dropping invalid APS frame without block number.");
                return None;
            };

            let Some(blocks) = NonZero::new(blocks) else {
                warn!("Dropping invalid APS frame with block number 0.");
                return None;
            };

            let (mut header, payload) = aps.into_parts();

            if blocks.get() == 1 {
                trace!("APS frame contains only 1 block.");
                header.drop_extended();
                return Some(Frame::raw(header, payload));
            }

            if let Some(previous_transaction) = self.transactions.insert(
                Index::new(source, header.counter()),
                Transaction::new(blocks, header, payload),
            ) {
                warn!("Dropping previous transaction: {previous_transaction:?}");
                return None;
            }

            trace!("Began new transaction for source: {source:?}");
            return None;
        }

        if extended
            .control()
            .contains(ExtendedControl::FOLLOWUP_FRAGMENT)
        {
            trace!("APS frame is followup fragment.");

            let Some(index) = extended.block_number() else {
                warn!("Dropping invalid APS frame without block number.");
                return None;
            };

            trace!("APS frame is is block #{index}");
            let (header, payload) = aps.into_parts();
            let key = Index::new(source, header.counter());

            let Some(transaction) = self.transactions.remove(&key) else {
                warn!("Dropping follow-up APS frame without existing transaction.");
                return None;
            };

            return match transaction.insert(index, payload) {
                InsertResult::Complete(frame) => {
                    trace!("Transaction complete.");
                    Some(frame)
                }
                InsertResult::Incomplete(transaction) => {
                    trace!("Transaction not yet complete.");
                    self.transactions.insert(key, transaction);
                    None
                }
                InsertResult::OutOfBounds(index) => {
                    warn!("Received out of bounds fragment: {index}. Dropping transaction.");
                    None
                }
            };
        }

        trace!("APS frame is not a follow-up fragment.");
        Some(aps)
    }
}
