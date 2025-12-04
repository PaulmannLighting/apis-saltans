use macaddr::MacAddr8;
use serde::Serialize;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Device {
    mac_address: MacAddr8,
    short_id: u16,
}

impl Device {
    #[must_use]
    pub const fn new(mac_address: MacAddr8, short_id: u16) -> Self {
        Self {
            mac_address,
            short_id,
        }
    }
}
