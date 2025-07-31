use crate::constants::U8_CAPACITY;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Command<const CAPACITY: usize = U8_CAPACITY> {
    attributes: heapless::Vec<u16, CAPACITY>,
}

impl<const CAPACITY: usize> Command<CAPACITY> {
    /// Create a new Read Attributes Command.
    #[must_use]
    pub const fn new(attributes: heapless::Vec<u16, CAPACITY>) -> Self {
        Self { attributes }
    }

    /// Add the respective attribute.
    pub fn add_attribute(&mut self, attribute: u16) -> Result<(), u16> {
        self.attributes.push(attribute)
    }

    /// Add the respective attribute and return `Self`.
    pub fn with_attribute(mut self, attribute: u16) -> Result<Self, u16> {
        self.add_attribute(attribute).map(|()| self)
    }
}

impl<const CAPACITY: usize> AsRef<[u16]> for Command<CAPACITY> {
    fn as_ref(&self) -> &[u16] {
        self.attributes.as_ref()
    }
}
