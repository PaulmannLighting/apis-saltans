use apis_saltans_core::IeeeAddress;

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
    #[must_use]
    pub const fn new(node_id: u16, ieee_address: Option<IeeeAddress>) -> Self {
        Self {
            node_id,
            ieee_address,
        }
    }

    #[must_use]
    pub const fn node_id(&self) -> u16 {
        self.node_id
    }

    #[must_use]
    pub const fn ieee_address(&self) -> Option<IeeeAddress> {
        self.ieee_address
    }

    #[must_use]
    pub const fn into_parts(self) -> (u16, Option<IeeeAddress>) {
        (self.node_id, self.ieee_address)
    }
}
