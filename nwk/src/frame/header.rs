use le_stream::ToLeStream;

/// Frame type.
#[derive(Debug)]
pub enum Header {
    /// ZCL header.
    Zcl(zcl::Header),
    /// ZDP transaction sequence number.
    Zdp(u8),
}

impl ToLeStream for Header {
    type Iter = Box<dyn Iterator<Item = u8>>;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::Zcl(header) => Box::new(header.to_le_stream()),
            Self::Zdp(transaction_seq) => Box::new(transaction_seq.to_le_stream()),
        }
    }
}
