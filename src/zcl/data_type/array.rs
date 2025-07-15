use std::slice::Chunks;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Array {
    size: u16,
    elements: Vec<u8>,
}

impl Array {
    /// Return an iterator over the elements of the array.
    pub fn items(&self) -> Chunks<'_, u8> {
        self.elements.chunks(usize::from(self.size))
    }
}
