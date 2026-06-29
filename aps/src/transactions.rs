use std::collections::BTreeMap;

use log::{debug, warn};

use crate::ExtendedControl;
use crate::data::{Frame, Header};

/// An APS data frame transaction.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Transactions {
    frames: BTreeMap<u8, (u8, Header, Vec<u8>)>,
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

            if let Some((_, header, payload)) = self
                .frames
                .insert(header.counter(), (blocks, header, payload))
            {
                warn!("Got a previous APS frame: {header:?}");
                return Some(Frame::raw(header, payload));
            }

            return None;
        }

        if extended
            .control()
            .contains(ExtendedControl::FOLLOWUP_FRAGMENT)
        {
            let Some(index) = extended.block_number() else {
                warn!("APS frame has no block number.");
                return Some(frame);
            };

            debug!("APS frame is follow-up fragment: block_number={index}");
            let (header, payload) = frame.into_parts();
            let (blocks, mut header, mut buffer) = self.frames.remove(&header.counter())?;
            buffer.extend(payload);

            if index.saturating_add(1) == blocks {
                debug!("APS frame complete: blocks={blocks}");
                header.drop_extended();
                return Some(Frame::raw(header, buffer));
            }

            debug!("APS frame incomplete: blocks={blocks}");
            self.frames
                .insert(header.counter(), (blocks, header, buffer));
            return None;
        }

        debug!("APS frame is not a follow-up fragment.");
        Some(frame)
    }
}
