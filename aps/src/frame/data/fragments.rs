use std::num::{NonZero, TryFromIntError};

use bytes::Bytes;

use crate::Fragmentation;
use crate::data::{Frame, Header};

const EMPTY_PAYLOAD_BLOCKS: u8 = 0;
const FIRST_FRAGMENT_INDEX: u8 = 0;
const SINGLE_BLOCK: u8 = 1;

/// Iterator over APS data fragments produced from a single frame payload.
///
/// Each yielded frame contains the next owned payload chunk and an extended
/// header that identifies whether the chunk is the first or a follow-up
/// fragment.
#[derive(Debug)]
pub struct Fragments {
    header: Header,
    payload: Bytes,
    chunk_size: NonZero<usize>,
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
    pub fn new(frame: Frame<Bytes>, chunk_size: NonZero<usize>) -> Result<Self, TryFromIntError> {
        let (header, payload) = frame.into_parts();
        let blocks: u8 = payload.len().div_ceil(chunk_size.get()).try_into()?;
        Ok(Self {
            header,
            payload,
            chunk_size,
            blocks,
            index: FIRST_FRAGMENT_INDEX,
        })
    }

    #[must_use]
    fn chunk_size(&self) -> usize {
        self.chunk_size.get().min(self.payload.len())
    }

    #[must_use]
    const fn fragmentation(&self) -> Fragmentation {
        if self.index == FIRST_FRAGMENT_INDEX {
            Fragmentation::First {
                blocks: self.blocks,
            }
        } else {
            Fragmentation::Followup { index: self.index }
        }
    }

    #[must_use]
    fn header(&self) -> Header {
        let mut header = self.header;
        header.set_fragmentation(self.fragmentation());
        header
    }
}

impl Iterator for Fragments {
    /// Fragmented APS data frame yielded by the iterator.
    type Item = Frame<Bytes>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == FIRST_FRAGMENT_INDEX && self.blocks == EMPTY_PAYLOAD_BLOCKS {
            self.index += 1;
            self.header.drop_extended();
            return Some(Frame::raw(self.header, self.payload.clone()));
        }

        if self.index >= self.blocks {
            return None;
        }

        let mut header = self.header();
        self.index += 1;

        if self.blocks == SINGLE_BLOCK {
            header.drop_extended();
            return Some(Frame::raw(header, self.payload.clone()));
        }

        let chunk = self.payload.split_to(self.chunk_size());
        Some(Frame::raw(header, chunk))
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZero;

    use bytes::Bytes;
    use zb_core::{Application, Endpoint, Profile};

    use super::{Fragments, SINGLE_BLOCK};
    use crate::data::{Frame, Header};
    use crate::{Destination, Extended};

    const CHUNK_SIZE: usize = 4;
    const CLUSTER_ID: u16 = 0x0006;
    const COUNTER: u8 = 0x2a;
    const FINAL_PARTIAL_CHUNK_LEN: usize = 2;
    const MULTI_BLOCK_PAYLOAD_LEN: usize = 10;
    const PROFILE_ID: u16 = Profile::ZigbeeHomeAutomation.as_u16();
    const SINGLE_BLOCK_PAYLOAD_LEN: usize = 3;

    fn chunk_size() -> NonZero<usize> {
        NonZero::new(CHUNK_SIZE).expect("chunk size is non-zero")
    }

    fn header(extended: Option<Extended>) -> Header {
        Header::new(
            Destination::Unicast(Application::default().into()),
            CLUSTER_ID,
            PROFILE_ID,
            Endpoint::default(),
            COUNTER,
            extended,
        )
    }

    #[test]
    fn empty_payload_yields_one_unfragmented_frame() {
        let frame = Frame::raw(
            header(Some(Extended::first_fragment(SINGLE_BLOCK))),
            Bytes::new(),
        );
        let mut fragments =
            Fragments::new(frame, chunk_size()).expect("empty payload has valid block count");

        let fragment = fragments.next().expect("empty payload is yielded once");

        assert!(fragment.payload().is_empty());
        assert_eq!(fragment.header().extended(), None);
        assert!(fragments.next().is_none());
    }

    #[test]
    fn single_block_payload_yields_one_unfragmented_frame() {
        let frame = Frame::raw(
            header(Some(Extended::first_fragment(SINGLE_BLOCK))),
            Bytes::from_static(&[0; SINGLE_BLOCK_PAYLOAD_LEN]),
        );
        let mut fragments = Fragments::new(frame, chunk_size())
            .expect("single block payload has valid block count");

        let fragment = fragments.next().expect("single block payload is yielded");

        assert_eq!(fragment.payload().len(), SINGLE_BLOCK_PAYLOAD_LEN);
        assert_eq!(fragment.header().extended(), None);
        assert!(fragments.next().is_none());
    }

    #[test]
    fn multi_block_payload_stops_after_final_partial_chunk() {
        let frame = Frame::raw(
            header(None),
            Bytes::from_static(&[0; MULTI_BLOCK_PAYLOAD_LEN]),
        );
        let fragments =
            Fragments::new(frame, chunk_size()).expect("multi block payload has valid block count");

        let payload_lengths: Vec<usize> =
            fragments.map(|fragment| fragment.payload().len()).collect();

        assert_eq!(
            payload_lengths,
            [CHUNK_SIZE, CHUNK_SIZE, FINAL_PARTIAL_CHUNK_LEN]
        );
    }
}
