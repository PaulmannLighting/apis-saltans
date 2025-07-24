use le_stream::derive::{FromLeStream, ToLeStream};
use le_stream::{Prefixed, WordSizedVec};

/// An octet string, with a capacity of [`u16::MAX`].
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct OctStr16(Prefixed<u16, WordSizedVec<u8>>);

impl AsRef<[u8]> for OctStr16 {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl TryFrom<&[u8]> for OctStr16 {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        WordSizedVec::try_from(value)
            .map(Prefixed::<u16, WordSizedVec<u8>>::new)
            .map(Self)
    }
}
