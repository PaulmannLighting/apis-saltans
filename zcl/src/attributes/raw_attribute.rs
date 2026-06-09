use zigbee::types::Type;

/// A raw attribute, which needs to be parsed into a specific type depending on the cluster it is in.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RawAttribute {
    id: u16,
    typ: Type,
}

impl RawAttribute {
    /// Create a new raw attribute.
    #[must_use]
    pub const fn new(id: u16, typ: Type) -> Self {
        Self { id, typ }
    }
}

impl From<(u16, Type)> for RawAttribute {
    fn from((id, typ): (u16, Type)) -> Self {
        Self { id, typ }
    }
}

impl From<RawAttribute> for (u16, Type) {
    fn from(value: RawAttribute) -> Self {
        (value.id, value.typ)
    }
}
