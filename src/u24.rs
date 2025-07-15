use std::num::TryFromIntError;

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct U24 {
    upper: u8,
    lower: u16,
}

impl U24 {
    /// Create a new `U42`.
    #[must_use]
    pub const fn new(upper: u8, lower: u16) -> Self {
        Self { upper, lower }
    }

    /// Return the bytes as big endian.
    #[must_use]
    pub const fn to_be_bytes(self) -> [u8; 3] {
        let [b, c] = self.lower.to_be_bytes();
        [self.upper, b, c]
    }

    /// Return the bytes as little endian.
    #[must_use]
    pub const fn to_le_bytes(self) -> [u8; 3] {
        let [a, b] = self.lower.to_le_bytes();
        [a, b, self.upper]
    }
}

impl From<U24> for u32 {
    fn from(u24: U24) -> Self {
        (Self::from(u24.upper) << 16) + (Self::from(u24.lower))
    }
}

impl TryFrom<u32> for U24 {
    type Error = TryFromIntError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Self {
            upper: u8::try_from(value >> 16)?,
            lower: u16::try_from(value & 0xFFFF)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::u24::U24;

    const N: U24 = U24::new(0xab, 0xcdef);

    #[test]
    fn test_be_bytes() {
        let bytes = N.to_be_bytes();
        assert_eq!(bytes, [0xab, 0xcd, 0xef]);
    }

    #[test]
    fn test_le_bytes() {
        let bytes = N.to_le_bytes();
        assert_eq!(bytes, [0xef, 0xcd, 0xab]);
    }

    #[test]
    fn test_u24_to_u32() {
        let n = u32::from(N);
        assert_eq!(n, 0x00ab_cdef);
    }

    #[test]
    fn test_u32_to_u24_ok() {
        let n = U24::try_from(0x00ab_cdef);
        assert_eq!(n, Ok(N));
    }

    #[test]
    fn test_u32_to_u24_err() {
        let n = U24::try_from(0x01ab_cdef);
        assert!(n.is_err());
    }
}
