use std::fmt::{self, Display};

use le_stream::ToLeStream;
use zb_core::Endpoint;

/// A destination that preserves raw endpoint IDs while parsing APS frames.
pub type WeakDestination = Destination<u8>;

/// Represents the destination of an APS frame.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Destination<E = Endpoint> {
    /// A unicast endpoint ID.
    Unicast(E),

    /// A broadcast endpoint ID.
    Broadcast(E),

    /// A group address.
    Group(u16),
}

impl<E> Display for Destination<E>
where
    E: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unicast(value) => write!(f, "Unicast({value})"),
            Self::Broadcast(value) => write!(f, "Broadcast({value})"),
            Self::Group(value) => write!(f, "Group({value})"),
        }
    }
}

impl From<Destination> for WeakDestination {
    fn from(destination: Destination) -> Self {
        match destination {
            Destination::Unicast(endpoint) => Self::Unicast(endpoint.into()),
            Destination::Broadcast(endpoint) => Self::Broadcast(endpoint.into()),
            Destination::Group(group) => Self::Group(group),
        }
    }
}

impl ToLeStream for WeakDestination {
    type Iter = iterator::DestinationIterator;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::Unicast(value) | Self::Broadcast(value) => value.into(),
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
