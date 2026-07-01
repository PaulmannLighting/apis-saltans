use le_stream::{FromLeStream, ToLeStream};

pub use self::clear_all_bindings_req_eui64::ClearAllBindingsReqEui64;
use crate::types::tlv::{General, Tag};

mod clear_all_bindings_req_eui64;

/// Local TLV structure.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Local {
    /// Clear All Bindings Request EUI64 List.
    ClearAllBindingsReqEui64(ClearAllBindingsReqEui64),
}

impl From<Local> for General {
    fn from(local: Local) -> Self {
        match local {
            Local::ClearAllBindingsReqEui64(value) => Self::serialize(value),
        }
    }
}

impl TryFrom<General> for Local {
    type Error = u8;

    fn try_from(general: General) -> Result<Self, Self::Error> {
        let (typ, payload) = general.into_parts();

        match typ {
            ClearAllBindingsReqEui64::TAG => {
                ClearAllBindingsReqEui64::from_le_stream(payload.into_iter())
                    .map(Self::ClearAllBindingsReqEui64)
                    .ok_or(typ)
            }
            typ => Err(typ),
        }
    }
}

impl ToLeStream for Local {
    type Iter = <General as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        General::from(self).to_le_stream()
    }
}
