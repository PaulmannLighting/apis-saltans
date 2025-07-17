use alloc::vec::Vec;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Command {
    attributes: Vec<u16>,
}

impl Command {
    /// Create a new Read Attributes Command.
    #[must_use]
    pub const fn new(attributes: Vec<u16>) -> Self {
        Self { attributes }
    }

    /// Add the respective attribute.
    pub fn add_attribute(&mut self, attribute: u16) {
        self.attributes.push(attribute);
    }

    /// Add the respective attribute and return `Self`.
    pub fn with_attribute(mut self, attribute: u16) -> Self {
        self.add_attribute(attribute);
        self
    }
}

impl AsRef<[u16]> for Command {
    fn as_ref(&self) -> &[u16] {
        self.attributes.as_ref()
    }
}
