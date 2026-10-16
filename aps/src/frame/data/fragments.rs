use core::num::{NonZero, TryFromIntError};

#[cfg(not(feature = "alloc"))]
use thiserror::Error;

use crate::data::{Frame, Header};
#[cfg(not(feature = "alloc"))]
use crate::{CapacityError, CapacityKind, FRAGMENT_CAPACITY, FRAGMENTS_CAPACITY};
use crate::{Fragmentation, Payload};

const EMPTY_PAYLOAD_BLOCKS: u8 = 0;
const FIRST_FRAGMENT_INDEX: u8 = 0;
const SINGLE_BLOCK: u8 = 1;

/// Fragmentation failure in allocating mode, preserving the original API.
#[cfg(feature = "alloc")]
pub type FragmentationError = TryFromIntError;

/// Fragmentation failure without an allocator.
#[cfg(not(feature = "alloc"))]
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum FragmentationError {
    /// The block count cannot be represented by the APS header.
    #[error("APS fragment count is out of range: {0}")]
    BlockCount(#[from] TryFromIntError),

    /// A configured capacity would be exceeded.
    #[error(transparent)]
    Capacity(#[from] CapacityError),
}

/// Iterator over APS data fragments produced from a single frame payload.
///
/// Each yielded frame contains the next owned payload chunk and an extended
/// header that identifies whether the chunk is the first or a follow-up
/// fragment.
#[derive(Debug)]
pub struct Fragments {
    header: Header,
    payload: Payload,
    chunk_size: NonZero<usize>,
    blocks: u8,
    index: u8,
    #[cfg(not(feature = "alloc"))]
    offset: usize,
}

impl Fragments {
    /// Create a fragment iterator for the given APS data frame.
    ///
    /// # Errors
    ///
    /// Returns an error if the number of fragments does not fit into the APS
    /// extended header block count field, or a bounded fragmentation capacity is exceeded.
    ///
    pub fn new(
        frame: Frame<Payload>,
        chunk_size: NonZero<usize>,
    ) -> Result<Self, FragmentationError> {
        let (header, payload) = frame.into_parts();
        let blocks: u8 = payload.len().div_ceil(chunk_size.get()).try_into()?;
        #[cfg(not(feature = "alloc"))]
        if blocks > SINGLE_BLOCK {
            let fragment_size = chunk_size.get().min(payload.len());
            if fragment_size > FRAGMENT_CAPACITY {
                return Err(CapacityError {
                    kind: CapacityKind::Fragment,
                    required: fragment_size,
                    capacity: FRAGMENT_CAPACITY,
                }
                .into());
            }
            if usize::from(blocks) > FRAGMENTS_CAPACITY {
                return Err(CapacityError {
                    kind: CapacityKind::Fragments,
                    required: usize::from(blocks),
                    capacity: FRAGMENTS_CAPACITY,
                }
                .into());
            }
        }
        Ok(Self {
            header,
            payload,
            chunk_size,
            blocks,
            index: FIRST_FRAGMENT_INDEX,
            #[cfg(not(feature = "alloc"))]
            offset: 0,
        })
    }

    #[cfg(feature = "alloc")]
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
    type Item = Frame<Payload>;

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

        #[cfg(feature = "alloc")]
        let chunk = self.payload.split_to(self.chunk_size());
        #[cfg(not(feature = "alloc"))]
        let chunk = {
            let end = self
                .offset
                .saturating_add(self.chunk_size.get())
                .min(self.payload.len());
            let chunk = Payload::try_from(&self.payload[self.offset..end])
                .expect("a slice of a bounded payload fits the same capacity");
            self.offset = end;
            chunk
        };
        Some(Frame::raw(header, chunk))
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use core::num::NonZero;

    use zb_core::{Application, Endpoint, Profile};

    use super::{Fragments, SINGLE_BLOCK};
    use crate::data::{DataHeaderParameters, Frame, Header};
    use crate::{Destination, Extended, Payload};

    const CHUNK_SIZE: usize = 4;
    const CLUSTER_ID: u16 = 0x0006;
    const COUNTER: u8 = 0x2a;
    const FINAL_PARTIAL_CHUNK_LEN: usize = 2;
    const MULTI_BLOCK_PAYLOAD_LEN: usize = 10;
    const PROFILE_ID: u16 = Profile::ZigbeeHomeAutomation.as_u16();
    const SINGLE_BLOCK_PAYLOAD_LEN: usize = 3;

    fn payload(bytes: &[u8]) -> Payload {
        #[cfg(feature = "alloc")]
        {
            Payload::copy_from_slice(bytes)
        }
        #[cfg(not(feature = "alloc"))]
        {
            Payload::try_from(bytes).expect("test payload fits")
        }
    }

    fn chunk_size() -> NonZero<usize> {
        NonZero::new(CHUNK_SIZE).expect("chunk size is non-zero")
    }

    fn header(extended: Option<Extended>) -> Header {
        Header::new(DataHeaderParameters {
            destination: Destination::Unicast(Application::default().into()),
            cluster_id: CLUSTER_ID,
            profile_id: PROFILE_ID,
            source_endpoint: Endpoint::default(),
            counter: COUNTER,
            extended,
        })
    }

    #[test]
    fn empty_payload_yields_one_unfragmented_frame() {
        let frame = Frame::raw(
            header(Some(Extended::first_fragment(SINGLE_BLOCK))),
            Payload::new(),
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
            payload(&[0; SINGLE_BLOCK_PAYLOAD_LEN]),
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
        let frame = Frame::raw(header(None), payload(&[0; MULTI_BLOCK_PAYLOAD_LEN]));
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
