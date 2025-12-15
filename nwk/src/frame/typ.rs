use le_stream::ToLeStream;
use zcl::Header;

/// Frame type.
#[derive(Debug)]
pub enum Type {
    Zcl(Header),
    Zdp(u8),
}

impl ToLeStream for Type {
    type Iter = Box<dyn Iterator<Item = u8>>;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Type::Zcl(header) => Box::new(header.to_le_stream()),
            Type::Zdp(transaction_seq) => Box::new(transaction_seq.to_le_stream()),
        }
    }
}
