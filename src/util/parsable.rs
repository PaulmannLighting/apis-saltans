use core::marker::PhantomData;

use le_stream::{FromLeStream, ToLeStream};

/// A wrapper type that allows parsing a source type into a destination type.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Parsable<Src, Dst> {
    src: Src,
    dst: PhantomData<Dst>,
}

impl<Src, Dst> Parsable<Src, Dst>
where
    Dst: TryFrom<Src, Error: Into<Src>>,
{
    /// Parse the source value into the destination type.
    pub fn parse(self) -> Result<Dst, Src> {
        Dst::try_from(self.src).map_err(Into::into)
    }
}

impl<Src, Dst> From<Dst> for Parsable<Src, Dst>
where
    Dst: Into<Src>,
{
    fn from(value: Dst) -> Self {
        Self {
            src: value.into(),
            dst: PhantomData,
        }
    }
}

impl<Src, Dst> FromLeStream for Parsable<Src, Dst>
where
    Src: FromLeStream,
{
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        Src::from_le_stream(bytes).map(|src| Self {
            src,
            dst: PhantomData,
        })
    }
}

impl<Src, Dst> ToLeStream for Parsable<Src, Dst>
where
    Src: ToLeStream,
{
    type Iter = <Src as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        self.src.to_le_stream()
    }
}
