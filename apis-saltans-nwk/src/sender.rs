use macaddr::MacAddr8;

#[cfg_attr(
    feature = "le-stream",
    derive(le_stream::FromLeStream, le_stream::ToLeStream)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Sender {
    node_id: u16,
    ieee_address: Option<MacAddr8>,
}

impl Sender {
    #[must_use]
    pub const fn new(node_id: u16, ieee_address: Option<MacAddr8>) -> Self {
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
    pub const fn ieee_address(&self) -> Option<MacAddr8> {
        self.ieee_address
    }

    #[must_use]
    pub const fn into_parts(self) -> (u16, Option<MacAddr8>) {
        (self.node_id, self.ieee_address)
    }
}
