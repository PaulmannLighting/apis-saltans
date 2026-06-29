use std::collections::BTreeMap;

use log::{debug, error, warn};

use crate::ExtendedControl;
use crate::data::{Frame, Header};

/// An APS data frame transaction.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Transactions {
    frames: BTreeMap<u8, (u8, Header, Vec<Vec<u8>>)>,
}

impl Transactions {
    /// Create a new transaction.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            frames: BTreeMap::new(),
        }
    }

    /// Add a frame to the transaction, returning the defragmented frame if possible.
    #[must_use]
    pub fn add(&mut self, frame: Frame<Vec<u8>>) -> Option<Frame<Vec<u8>>> {
        debug!("Received APS frame: {frame:?}");

        let Some(extended) = frame.header().extended() else {
            debug!("APS frame has no extended header.");
            return Some(frame);
        };

        debug!("APS frame has extended header: {extended:?}");

        if extended.control().contains(ExtendedControl::FIRST_FRAGMENT) {
            debug!("APS frame is first fragment.");

            let Some(blocks) = extended.block_number() else {
                warn!("APS frame has no block number.");
                return Some(frame);
            };

            let (header, payload) = frame.into_parts();
            let mut chunks = vec![Vec::new(); blocks.into()];

            if let Some(entry) = chunks.get_mut(0) {
                entry.extend(payload);
            } else {
                error!("Invalid block number: {blocks}");
            }

            debug!("Adding APS frame to transaction: blocks={blocks}");
            self.frames
                .insert(header.counter(), (blocks, header, chunks));
            return None;
        }

        if !extended
            .control()
            .contains(ExtendedControl::FOLLOWUP_FRAGMENT)
        {
            debug!("APS frame is not a follow-up fragment.");
            return Some(frame);
        }

        let Some(index) = extended.block_number() else {
            warn!("APS frame has no block number.");
            return Some(frame);
        };

        debug!("APS frame is follow-up fragment: block_number={index}");
        let (header, payload) = frame.into_parts();
        let (blocks, mut header, mut chunks) = self.frames.remove(&header.counter())?;

        if let Some(entry) = chunks.get_mut(usize::from(index)) {
            entry.extend(payload);
        } else {
            error!("Invalid block number: {index}");
            return None;
        }

        if chunks.len() == blocks.saturating_sub(1).into() {
            debug!("APS frame complete: blocks={blocks}");
            header.drop_extended();
            return Some(Frame::raw(header, chunks.into_iter().flatten().collect()));
        }

        debug!("APS frame incomplete: blocks={blocks}");
        self.frames
            .insert(header.counter(), (blocks, header, chunks));
        None
    }
}
