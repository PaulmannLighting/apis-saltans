use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use macaddr::MacAddr8;
use zigbee::{Address, Application};

pub use self::endpoint::{Cluster, Endpoint};

mod endpoint;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Device {
    ieee_address: MacAddr8,
    short_id: u16,
    endpoints: BTreeMap<Application, Endpoint>,
    last_seen: Option<DateTime<Utc>>,
}

impl Device {
    /// Create a new device.
    #[must_use]
    pub const fn new(
        ieee_address: MacAddr8,
        short_id: u16,
        endpoints: BTreeMap<Application, Endpoint>,
        last_seen: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            ieee_address,
            short_id,
            endpoints,
            last_seen,
        }
    }

    #[must_use]
    pub const fn ieee_address(&self) -> MacAddr8 {
        self.ieee_address
    }

    #[must_use]
    pub const fn short_id(&self) -> u16 {
        self.short_id
    }

    pub const fn set_short_id(&mut self, short_id: u16) {
        self.short_id = short_id;
    }

    #[must_use]
    pub const fn endpoints(&self) -> &BTreeMap<Application, Endpoint> {
        &self.endpoints
    }

    pub const fn endpoints_mut(&mut self) -> &mut BTreeMap<Application, Endpoint> {
        &mut self.endpoints
    }

    #[must_use]
    pub const fn last_seen(&self) -> Option<DateTime<Utc>> {
        self.last_seen
    }

    pub const fn set_last_seen(&mut self, last_seen: DateTime<Utc>) {
        self.last_seen.replace(last_seen);
    }

    pub fn last_seen_now(&mut self) {
        self.set_last_seen(Utc::now());
    }
}

impl From<Address> for Device {
    fn from(address: Address) -> Self {
        Self::new(
            address.ieee_address(),
            address.short_id(),
            BTreeMap::default(),
            Some(Utc::now()),
        )
    }
}
