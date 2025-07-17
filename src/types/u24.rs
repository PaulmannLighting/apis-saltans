use std::array::IntoIter;

use le_stream::{FromLeStream, ToLeStream};

/// A 24-bit unsigned integer type.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct U24(u32);

impl U24 {
    /// The minimum value for `U24`, which is 0.
    #[allow(unsafe_code)]
    // SAFETY: This constant is safe because it is guaranteed to be within the valid range of U24.
    pub const MIN: Self = unsafe { Self::new_unchecked(0x0000_0000) };

    /// The maximum value for `U24`, which is 16,777,215 (`0x00FF_FFFF`).
    #[allow(unsafe_code)]
    // SAFETY: This constant is safe because it is guaranteed to be within the valid range of U24.
    pub const MAX: Self = unsafe { Self::new_unchecked(0x00FF_FFFF) };

    /// Create a new `U24` value.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `value` is within the range of 0 to 16,777,215 (`0x00FF_FFFF`).
    #[must_use]
    #[allow(unsafe_code)]
    pub const unsafe fn new_unchecked(value: u32) -> Self {
        Self(value)
    }

    /// Create a new `U24` value if it is within the valid range.
    #[must_use]
    pub const fn new(value: u32) -> Option<Self> {
        if value <= Self::MAX.into_u32() {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Return the underlying `u32` value.
    #[must_use]
    pub const fn into_u32(self) -> u32 {
        self.0
    }
}

impl From<u8> for U24 {
    fn from(value: u8) -> Self {
        #[allow(unsafe_code)]
        // SAFETY: A `u8` value is always within the range of `U24`, so this is safe.
        unsafe {
            Self::new_unchecked(u32::from(value))
        }
    }
}

impl From<u16> for U24 {
    fn from(value: u16) -> Self {
        #[allow(unsafe_code)]
        // SAFETY: A `u16` value is always within the range of `U24`, so this is safe.
        unsafe {
            Self::new_unchecked(u32::from(value))
        }
    }
}

impl From<U24> for u32 {
    fn from(value: U24) -> Self {
        value.into_u32()
    }
}

impl TryFrom<u32> for U24 {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(value)
    }
}

impl FromLeStream for U24 {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let bytes = [bytes.next()?, bytes.next()?, bytes.next()?, 0x00];
        #[allow(unsafe_code)]
        // SAFETY: The bytes are guaranteed to be valid for creating a `U24` value.
        let inner = unsafe { Self::new_unchecked(u32::from_le_bytes(bytes)) };
        Some(inner)
    }
}

impl ToLeStream for U24 {
    type Iter = IntoIter<u8, 3>;

    fn to_le_stream(self) -> Self::Iter {
        let [a, b, c, _] = self.0.to_le_bytes();
        [a, b, c].into_iter()
    }
}
