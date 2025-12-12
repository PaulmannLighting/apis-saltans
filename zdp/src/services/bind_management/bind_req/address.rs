use std::fmt::Display;

use le_stream::ToLeStream;
use macaddr::MacAddr8;

use self::iterator::AddressLeStream;

/// Address type for Bind Request.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Address {
    /// 16-bit group address.
    Group(u16),
    /// 64-bit extended address.
    Extended(MacAddr8),
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Group(addr) => write!(f, "{addr:#06X}"),
            Self::Extended(addr) => write!(f, "{addr}"),
        }
    }
}

impl ToLeStream for Address {
    type Iter = AddressLeStream;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::Group(addr) => AddressLeStream::Group(addr.to_le_stream()),
            Self::Extended(addr) => AddressLeStream::Extended(addr.to_le_stream()),
        }
    }
}

mod iterator {
    use le_stream::ToLeStream;
    use macaddr::MacAddr8;

    /// Iterator for little-endian stream of `Address`.
    pub enum AddressLeStream {
        Group(<u16 as ToLeStream>::Iter),
        Extended(<MacAddr8 as ToLeStream>::Iter),
    }

    impl Iterator for AddressLeStream {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::Group(iter) => iter.next(),
                Self::Extended(iter) => iter.next(),
            }
        }
    }
}
