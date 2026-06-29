use std::collections::BTreeMap;

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
        let Some(extended) = frame.header().extended() else {
            return Some(frame);
        };

        if extended.control().contains(ExtendedControl::FIRST_FRAGMENT) {
            let Some(blocks) = extended.block_number() else {
                return Some(frame);
            };

            let (header, payload) = frame.into_parts();
            let mut chunks = Vec::with_capacity(blocks.into());
            chunks.push(payload);

            self.frames
                .insert(header.counter(), (blocks, header, chunks));
            return None;
        }

        if !extended
            .control()
            .contains(ExtendedControl::FOLLOWUP_FRAGMENT)
        {
            return Some(frame);
        }

        let Some(index) = extended.block_number() else {
            return Some(frame);
        };

        let (header, payload) = frame.into_parts();
        let (blocks, mut header, mut chunks) = self.frames.remove(&header.counter())?;

        if let Some(entry) = chunks.get_mut(usize::from(index)) {
            entry.extend(payload);
        } else {
            return None;
        }

        if chunks.len() == blocks.saturating_sub(1).into() {
            header.drop_extended();
            return Some(Frame::raw(header, chunks.into_iter().flatten().collect()));
        }

        self.frames
            .insert(header.counter(), (blocks, header, chunks));
        None
    }
}
