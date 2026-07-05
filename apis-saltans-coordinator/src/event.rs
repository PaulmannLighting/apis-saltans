use apis_saltans_aps::Data;
use apis_saltans_core::{Address, Endpoint};
use apis_saltans_zcl::{Cluster, Frame};

pub use self::r#type::Type;

mod r#type;

/// A generic Zigbee event.
#[derive(Clone, Debug)]
pub struct Event {
    src_address: Address,
    src_endpoint: Endpoint,
    typ: Type,
}

impl Event {
    /// Create a new event.
    #[must_use]
    pub(crate) fn new(src_address: Address, aps: Data<Frame<Cluster>>) -> Self {
        Self {
            src_address,
            src_endpoint: aps.header().source_endpoint(),
            typ: aps.into(),
        }
    }

    /// Return the source address of the event.
    #[must_use]
    pub const fn src_address(&self) -> &Address {
        &self.src_address
    }

    /// Return the source endpoint of the event.
    #[must_use]
    pub const fn src_endpoint(&self) -> Endpoint {
        self.src_endpoint
    }

    /// Return the type of event.
    #[must_use]
    pub const fn typ(&self) -> &Type {
        &self.typ
    }

    /// Return the parts of the event.
    #[must_use]
    pub fn into_parts(self) -> (Address, Endpoint, Type) {
        (self.src_address, self.src_endpoint, self.typ)
    }
}
