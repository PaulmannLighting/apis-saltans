use std::collections::BTreeMap;

use crate::ExtendedControl;
use crate::data::Frame;

/// An APS data frame transaction.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Transaction {
    frames: BTreeMap<u8, (u8, Frame<Vec<u8>>)>,
}

impl Transaction {
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

            self.frames
                .insert(frame.header().counter(), (blocks, frame));
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
        let (blocks, mut parent) = self.frames.remove(&header.counter())?;
        parent.extend(payload);

        if index == blocks.saturating_sub(1) {
            parent.drop_extended();
            return Some(parent);
        }

        self.frames.insert(header.counter(), (blocks, parent));
        None
    }
}
