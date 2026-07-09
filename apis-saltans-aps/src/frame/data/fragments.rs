use std::num::TryFromIntError;

use bytes::Bytes;

use crate::Fragmentation;
use crate::data::{Frame, Header};

/// Iterator over APS data fragments produced from a single frame payload.
///
/// Each yielded frame contains the next owned payload chunk and an extended
/// header that identifies whether the chunk is the first or a follow-up
/// fragment.
#[derive(Debug)]
pub struct Fragments {
    header: Header,
    payload: Bytes,
    chunk_size: usize,
    blocks: u8,
    index: u8,
}

impl Fragments {
    /// Create a fragment iterator for the given APS data frame.
    ///
    /// # Errors
    ///
    /// Returns an error if the number of fragments does not fit into the APS
    /// extended header block count field.
    ///
    /// # Panics
    ///
    /// Panics if `chunk_size` is zero.
    pub fn new(frame: Frame<Bytes>, chunk_size: usize) -> Result<Self, TryFromIntError> {
        let (header, payload) = frame.into_parts();
        let blocks: u8 = payload.len().div_ceil(chunk_size).try_into()?;
        Ok(Self {
            header,
            payload,
            chunk_size,
            blocks,
            index: 0,
        })
    }

    #[must_use]
    const fn fragmentation(&self) -> Fragmentation {
        if self.index == 0 {
            Fragmentation::First {
                blocks: self.blocks,
            }
        } else {
            Fragmentation::Followup { index: self.index }
        }
    }

    #[must_use]
    fn next_header(&mut self) -> Header {
        let mut header = self.header;
        header.set_fragmentation(self.fragmentation());
        self.index += 1;
        header
    }
}

impl Iterator for Fragments {
    /// Fragmented APS data frame yielded by the iterator.
    type Item = Frame<Bytes>;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self.payload.split_to(self.chunk_size);

        if chunk.is_empty() {
            return None;
        }

        // Shortcut for frames which don't need fragmentation.
        if self.blocks == 1 {
            return Some(Frame::raw(self.header, chunk));
        }

        Some(Frame::raw(self.next_header(), chunk))
    }
}
