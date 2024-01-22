use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Address {
    address: u8,
    endpoint: u8,
}

impl Address {
    #[must_use]
    pub const fn new(address: u8, endpoint: u8) -> Self {
        Self { address, endpoint }
    }

    #[must_use]
    pub const fn address(&self) -> u8 {
        self.address
    }

    #[must_use]
    pub const fn endpoint(&self) -> u8 {
        self.endpoint
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#04X}/{}", self.address, self.endpoint)
    }
}

impl From<u8> for Address {
    fn from(address: u8) -> Self {
        Address::new(address, 0)
    }
}
