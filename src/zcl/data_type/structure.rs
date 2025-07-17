use alloc::vec::Vec;
use core::slice::Chunks;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Structure {
    size: u16,
    elements: Vec<u8>,
}

impl Structure {
    /// Return an iterator over the elements of the structure.
    pub fn items(&self) -> Chunks<'_, u8> {
        self.elements.chunks(usize::from(self.size))
    }
}
