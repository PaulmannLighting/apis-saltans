pub use self::attributes::Attributes;

mod attributes;

/// State of a cluster.
#[derive(Debug, Default)]
pub struct Cluster {
    attributes: Option<Attributes>,
}

impl Cluster {
    #[must_use]
    pub fn attributes(&self) -> Option<&Attributes> {
        self.attributes.as_ref()
    }

    pub fn set_attributes(&mut self, attributes: Attributes) {
        self.attributes.replace(attributes);
    }
}
