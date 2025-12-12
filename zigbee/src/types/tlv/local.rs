mod clear_all_bindings_req_eui64;

use le_stream::{FromLeStream, FromLeStreamTagged, ToLeStream};

pub use self::clear_all_bindings_req_eui64::ClearAllBindingsReqEui64;
use self::iter::LocalIter;
use crate::types::tlv::Tag;

/// Local TLV structure.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Local {
    /// Clear All Bindings Request EUI64 List.
    ClearAllBindingsReqEui64(ClearAllBindingsReqEui64),
}

impl FromLeStreamTagged for Local {
    type Tag = u8;

    fn from_le_stream_tagged<T>(tag: Self::Tag, bytes: T) -> Result<Option<Self>, Self::Tag>
    where
        T: Iterator<Item = u8>,
    {
        match tag {
            ClearAllBindingsReqEui64::TAG => {
                Ok(ClearAllBindingsReqEui64::from_le_stream(bytes)
                    .map(Self::ClearAllBindingsReqEui64))
            }
            _ => Err(tag),
        }
    }
}

impl ToLeStream for Local {
    type Iter = LocalIter;

    fn to_le_stream(self) -> Self::Iter {
        self.into()
    }
}

mod iter {
    use le_stream::ToLeStream;

    use crate::types::tlv::Local;
    use crate::types::tlv::local::ClearAllBindingsReqEui64;

    pub enum LocalIter {
        ClearAllBindingsReqEui64(<ClearAllBindingsReqEui64 as ToLeStream>::Iter),
    }

    impl Iterator for LocalIter {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::ClearAllBindingsReqEui64(iter) => iter.next(),
            }
        }
    }

    impl From<Local> for LocalIter {
        fn from(local: Local) -> Self {
            match local {
                Local::ClearAllBindingsReqEui64(value) => {
                    Self::ClearAllBindingsReqEui64(value.to_le_stream())
                }
            }
        }
    }
}
