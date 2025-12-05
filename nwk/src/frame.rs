use le_stream::ToLeStream;
use zcl::Header;

/// A non-generic view on a ZCL frame for transmission via channels.
#[derive(Debug, ToLeStream)]
pub struct Frame {
    header: Header,
    payload: Box<[u8]>,
}

impl Frame {
    /// Return the header of the ZCL frame.
    #[must_use]
    pub const fn header(&self) -> &Header {
        &self.header
    }

    /// Set the sequence number of the ZCL frame.
    pub const fn set_seq(&mut self, seq: u8) {
        self.header.set_seq(seq);
    }

    /// Return the payload of the ZCL frame.
    #[must_use]
    pub const fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// Serialize the ZCL frame into a little-endian byte array.
    #[must_use]
    pub fn serialize(self) -> Box<[u8]> {
        self.to_le_stream().collect()
    }
}

impl<T> From<zcl::Frame<T>> for Frame
where
    T: ToLeStream,
{
    fn from(frame: zcl::Frame<T>) -> Self {
        let (header, payload) = frame.into_parts();
        Self {
            header,
            payload: payload.to_le_stream().collect(),
        }
    }
}
