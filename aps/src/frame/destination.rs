use std::fmt::{self, Display};

use le_stream::ToLeStream;
use zb_core::GroupId;
use zb_core::endpoint::{Application, Broadcast};

/// A variant of `Destination` with weaker invariants to allow graceful parsing of APS frames.
pub type WeakDestination = Destination<u8, u8, u16>;

/// Represents the destination of an APS frame.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Destination<U = Application, B = Broadcast, G = GroupId> {
    /// A unicast endpoint ID.
    Unicast(U),

    /// A broadcast endpoint ID.
    Broadcast(B),

    /// A group address.
    Group(G),
}

impl<U, B, G> Display for Destination<U, B, G>
where
    U: Display,
    B: Display,
    G: Display,
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
            Destination::Unicast(device) => Self::Unicast(device.into()),
            Destination::Broadcast(broadcast) => Self::Broadcast(broadcast.into()),
            Destination::Group(group_id) => Self::Group(group_id.into()),
        }
    }
}

impl From<zb_core::Destination> for WeakDestination {
    fn from(destination: zb_core::Destination) -> Self {
        match destination {
            zb_core::Destination::Device(device) => Self::Unicast(device.endpoint().into()),
            zb_core::Destination::Broadcast(broadcast) => {
                Self::Broadcast(broadcast.endpoint().into())
            }
            zb_core::Destination::Group(group_id) => Self::Group(group_id.into()),
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
