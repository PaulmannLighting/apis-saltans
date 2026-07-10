use apis_saltans_aps::Data;
use apis_saltans_core::endpoint::Reserved;
use apis_saltans_core::{Endpoint, FullAddress};
use apis_saltans_zcl::{Cluster, Frame};

pub use self::r#type::Type;

mod r#type;

/// A generic Zigbee event.
#[derive(Clone, Debug)]
pub struct Event {
    src_address: FullAddress,
    src_endpoint: Result<Endpoint, Reserved>,
    typ: Type,
}

impl Event {
    /// Create a new event.
    pub(crate) fn new(src_address: FullAddress, aps: Data<Frame<Cluster>>) -> Self {
        Self {
            src_address,
            src_endpoint: aps.header().source_endpoint(),
            typ: aps.into(),
        }
    }

    #[must_use]
    pub const fn src_address(&self) -> FullAddress {
        self.src_address
    }

    /// Return the source endpoint of the event.
    pub const fn src_endpoint(&self) -> Result<Endpoint, Reserved> {
        self.src_endpoint
    }

    /// Return the type of event.
    #[must_use]
    pub const fn typ(&self) -> &Type {
        &self.typ
    }

    /// Return the parts of the event.
    #[must_use]
    pub fn into_type(self) -> Type {
        self.typ
    }
}
