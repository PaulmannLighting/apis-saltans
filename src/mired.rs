const NUMERATOR: u16 = 10.pow(6);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mired(u16);

impl Mired {
    #[must_use]
    pub const fn new(mired: u16) -> Self {
        Self(mired)
    }

    #[must_use]
    pub const fn from_kelvin(kelvin: u16) -> Self {
        Self(NUMERATOR / kelvin)
    }

    #[must_use]
    pub const fn to_kelvin(&self) -> u16 {
        NUMERATOR / self.0
    }
}

impl From<u16> for Mired {
    fn from(mired: u16) -> Self {
        Self(mired)
    }
}

impl From<Mired> for u16 {
    fn from(mired: Mired) -> Self {
        mired.0
    }
}
