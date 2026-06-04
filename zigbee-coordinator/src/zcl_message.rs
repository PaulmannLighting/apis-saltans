use aps::data::Header;
use zcl::{Cluster, Frame};

/// An incoming message from the ZCL.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ZclMessage {
    src_address: u16,
    aps_header: Header,
    frame: Frame<Cluster>,
}

impl ZclMessage {
    /// Create a new ZCL message.
    #[must_use]
    pub const fn new(src_address: u16, aps_header: Header, frame: Frame<Cluster>) -> Self {
        Self {
            src_address,
            aps_header,
            frame,
        }
    }

    /// Return the source address.
    #[must_use]
    pub const fn src_address(&self) -> u16 {
        self.src_address
    }

    /// Returns the APS header.
    #[must_use]
    pub const fn aps_header(&self) -> &Header {
        &self.aps_header
    }

    /// Return the ZCL frame.
    #[must_use]
    pub const fn frame(&self) -> &Frame<Cluster> {
        &self.frame
    }

    /// Destructure message into its constituent parts.
    #[must_use]
    pub fn into_parts(self) -> (u16, Header, Frame<Cluster>) {
        (self.src_address, self.aps_header, self.frame)
    }
}
