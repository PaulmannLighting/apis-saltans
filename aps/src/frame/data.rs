//! APS Data frame definitions.

use zigbee::Endpoint;

pub use self::header::Header;
pub use self::unicast::Unicast;
use crate::Extended;
use crate::frame::destination::Destination;

mod header;
mod unicast;

/// An APS Data frame.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Frame<T> {
    header: Header,
    payload: T,
}

impl<T> Frame<T> {
    /// Creates a new APS Data frame header without any validation.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided header is consistent with the payload.
    #[expect(unsafe_code)]
    #[must_use]
    pub const unsafe fn new_unchecked(header: Header, payload: T) -> Self {
        Self { header, payload }
    }

    /// Creates a new APS Data frame header.
    #[must_use]
    pub fn new(
        destination: Destination,
        cluster_id: u16,
        profile_id: u16,
        source_endpoint: Endpoint,
        counter: u8,
        extended: Option<Extended>,
        payload: T,
    ) -> Self {
        Self {
            header: Header::new(
                destination,
                cluster_id,
                profile_id,
                source_endpoint,
                counter,
                extended,
            ),
            payload,
        }
    }

    /// Return a reference to the header.
    #[must_use]
    pub const fn header(&self) -> &Header {
        &self.header
    }

    /// Return a reference to the payload.
    #[must_use]
    pub const fn payload(&self) -> &T {
        &self.payload
    }

    /// Return the header and payload, consuming the frame.
    #[must_use]
    pub fn into_parts(self) -> (Header, T) {
        (self.header, self.payload)
    }
}

impl<T> From<Unicast<T>> for Frame<T> {
    fn from(unicast: Unicast<T>) -> Self {
        let (header, payload) = unicast.into_parts();

        Self {
            header: header.into(),
            payload,
        }
    }
}

impl Frame<Vec<u8>> {
    /// Return a new frame with the given header and payload.
    #[must_use]
    pub const fn raw(header: Header, payload: Vec<u8>) -> Self {
        Self { header, payload }
    }

    /// Extend the payload with the given data.
    pub fn extend<T>(&mut self, data: T)
    where
        T: IntoIterator<Item = u8>,
    {
        self.payload.extend(data);
    }

    /// Drop the extended header.
    pub const fn drop_extended(&mut self) {
        self.header.drop_extended();
    }

    /// Parse the frame into a frame with typed payload.
    ///
    /// # Errors
    ///
    /// Returns an error if the payload cannot be parsed into the given type.
    pub fn parse<T>(self) -> Result<Frame<T>, T::Error>
    where
        T: TryFrom<Self>,
    {
        let header = self.header.clone();
        T::try_from(self).map(|payload| Frame { header, payload })
    }
}
