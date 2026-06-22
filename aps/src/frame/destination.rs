use le_stream::ToLeStream;
use zigbee::Endpoint;

use self::iterator::DestinationIterator;

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

impl ToLeStream for Destination {
    type Iter = DestinationIterator;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::Unicast(value) | Self::Broadcast(value) => value.into(),
            Self::Group(value) => value.into(),
        }
    }
}

mod iterator {
    use le_stream::ToLeStream;
    use zigbee::Endpoint;

    /// Le-stream iterator
    pub enum DestinationIterator {
        Endpoint(<Endpoint as ToLeStream>::Iter),
        U16(<u16 as ToLeStream>::Iter),
    }

    impl From<Endpoint> for DestinationIterator {
        fn from(value: Endpoint) -> Self {
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
