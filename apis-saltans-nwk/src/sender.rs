use apis_saltans_core::IeeeAddress;

/// Network-layer sender information.
///
/// The sender always includes the 16-bit network address. The IEEE address is
/// optional because it may not be known for every incoming frame.
#[cfg_attr(
    feature = "le-stream",
    derive(le_stream::FromLeStream, le_stream::ToLeStream)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Sender {
    node_id: u16,
    ieee_address: Option<IeeeAddress>,
}

impl Sender {
    /// Create sender information from a network address and optional IEEE address.
    #[must_use]
    pub const fn new(node_id: u16, ieee_address: Option<IeeeAddress>) -> Self {
        Self {
            node_id,
            ieee_address,
        }
    }

    /// Return the 16-bit network address.
    #[must_use]
    pub const fn node_id(&self) -> u16 {
        self.node_id
    }

    /// Return the IEEE address of the sender, if known.
    #[must_use]
    pub const fn ieee_address(&self) -> Option<IeeeAddress> {
        self.ieee_address
    }

    /// Split the sender into its network address and optional IEEE address.
    #[must_use]
    pub const fn into_parts(self) -> (u16, Option<IeeeAddress>) {
        (self.node_id, self.ieee_address)
    }
}
