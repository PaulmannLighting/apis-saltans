use std::fmt::{self, Display};

use le_stream::ToLeStream;
use zb_core::Endpoint;

/// Represents the destination of an APS frame.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Destination {
    /// A unicast endpoint ID.
    Unicast(Endpoint),

    /// A broadcast endpoint ID.
    Broadcast(Endpoint),

    /// A group address.
    Group(u16),
}

impl Display for Destination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unicast(value) => write!(f, "Unicast({value})"),
            Self::Broadcast(value) => write!(f, "Broadcast({value})"),
            Self::Group(value) => write!(f, "Group({value})"),
        }
    }
}

impl From<zb_core::Destination> for Destination {
    fn from(destination: zb_core::Destination) -> Self {
        match destination {
            zb_core::Destination::Device(device) => Self::Unicast(device.endpoint()),
            zb_core::Destination::Broadcast(broadcast) => Self::Broadcast(broadcast.endpoint()),
            zb_core::Destination::Group(group_id) => Self::Group(group_id.into()),
        }
    }
}

impl ToLeStream for Destination {
    type Iter = iterator::DestinationIterator;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::Unicast(value) | Self::Broadcast(value) => value.as_u8().into(),
            Self::Group(value) => value.into(),
        }
    }
}

mod iterator {
    use le_stream::ToLeStream;

    /// Le-stream iterator
    pub enum DestinationIterator {
        Endpoint(<u8 as ToLeStream>::Iter),
        U16(<u16 as ToLeStream>::Iter),
    }

    impl From<u8> for DestinationIterator {
        fn from(value: u8) -> Self {
            Self::Endpoint(value.to_le_stream())
        }
    }

    impl From<u16> for DestinationIterator {
        fn from(value: u16) -> Self {
            Self::U16(value.to_le_stream())
        }
    }

    impl Iterator for DestinationIterator {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::Endpoint(iter) => iter.next(),
                Self::U16(iter) => iter.next(),
            }
        }
    }
}
