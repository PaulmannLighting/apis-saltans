use std::fmt::Display;

use le_stream::{FromLeStream, ToLeStream};
use macaddr::MacAddr8;
use zigbee::Cluster;
use zigbee::types::tlv::{Local, Tlv};

use crate::Service;

/// Clear All Bindings Request
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct ClearAllBindingsReq {
    tlvs: Vec<Tlv>,
}

impl ClearAllBindingsReq {
    /// Creates a new `ClearAllBindingsReq`.
    #[must_use]
    pub const fn new(tlvs: Vec<Tlv>) -> Self {
        Self { tlvs }
    }

    /// Returns the provided TLVs.
    #[must_use]
    pub fn tlvs(&self) -> &[Tlv] {
        &self.tlvs
    }

    /// Returns an iterator over all EUI64s in the Clear All Bindings Request.
    pub fn eui64s(&self) -> impl Iterator<Item = &'_ MacAddr8> {
        self.tlvs
            .iter()
            .filter_map(|tlv| {
                if let Tlv::Local(Local::ClearAllBindingsReqEui64(clear_all_bindings_req_eui64)) =
                    tlv
                {
                    Some(clear_all_bindings_req_eui64.eui64s())
                } else {
                    None
                }
            })
            .flatten()
    }
}

impl Cluster for ClearAllBindingsReq {
    const ID: u16 = 0x002b;
}

impl Service for ClearAllBindingsReq {
    const NAME: &'static str = "Clear_All_Bindings_req";
}

impl Display for ClearAllBindingsReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {{ tlvs: {:?} }}", Self::NAME, self.tlvs)
    }
}
