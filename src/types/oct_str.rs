use le_stream::derive::{FromLeStream, ToLeStream};
use le_stream::{ByteSizedVec, Prefixed};

/// An octet string, with a capacity of [`u8::MAX`].
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct OctStr(Prefixed<u8, ByteSizedVec<u8>>);

impl AsRef<[u8]> for OctStr {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl TryFrom<&[u8]> for OctStr {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        ByteSizedVec::try_from(value)
            .map(Prefixed::<u8, ByteSizedVec<u8>>::new)
            .map(Self)
    }
}
