use le_stream::ToLeStream;

use self::iterator::DestinationIterator;

/// Represents the destination of an APS frame.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Destination {
    /// A unicast endpoint ID.
    Unicast(u8),
    /// A broadcast endpoint ID.
    Broadcast(u8),
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

    /// Le-stream iterator
    pub enum DestinationIterator {
        U8(<u8 as ToLeStream>::Iter),
        U16(<u16 as ToLeStream>::Iter),
    }

    impl From<u8> for DestinationIterator {
        fn from(value: u8) -> Self {
            Self::U8(value.to_le_stream())
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
                DestinationIterator::U8(iter) => iter.next(),
                DestinationIterator::U16(iter) => iter.next(),
            }
        }
    }
}
