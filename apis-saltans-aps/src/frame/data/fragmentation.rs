use std::num::TryFromIntError;

use bytes::Bytes;

use crate::Fragmentation;
use crate::data::{Frame, Header};

#[derive(Debug)]
pub struct Fragments {
    header: Header,
    payload: Bytes,
    chunk_size: usize,
    blocks: u8,
    index: u8,
}

impl Fragments {
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
    fn fragmentation(&self) -> Fragmentation {
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
    type Item = Frame<Bytes>;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self.payload.split_to(self.chunk_size);
        Some(Frame::raw(self.next_header(), chunk))
    }
}
