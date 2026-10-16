use heapless::Vec;

use super::WIRE_CAPACITY;

/// Fixed scratch buffer for serializers whose protocol maximum is 269 bytes.
///
/// Its capacity is checked at compile time. This buffer is not used for
/// arbitrary-length commands or records.
#[derive(Debug, Default)]
pub struct WireBytes(Vec<u8, WIRE_CAPACITY>);

impl WireBytes {
    pub(crate) const fn new() -> Self {
        Self(Vec::new())
    }
    pub(crate) fn push(&mut self, byte: u8) {
        self.0
            .push(byte)
            .expect("fixed protocol payload fits the validated wire capacity");
    }
}
impl Extend<u8> for WireBytes {
    fn extend<T>(&mut self, bytes: T)
    where
        T: IntoIterator<Item = u8>,
    {
        for byte in bytes {
            self.push(byte);
        }
    }
}
impl IntoIterator for WireBytes {
    type Item = u8;
    type IntoIter = <Vec<u8, WIRE_CAPACITY> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
