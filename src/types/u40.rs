use core::array::IntoIter;

use le_stream::{FromLeStream, ToLeStream};

/// A 24-bit unsigned integer type.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct U40(u64);

impl U40 {
    /// The minimum value for `U40`, which is 0.
    #[allow(unsafe_code)]
    // SAFETY: This constant is safe because it is guaranteed to be within the valid range of U40.
    pub const MIN: Self = unsafe { Self::new_unchecked(0x0000_0000_0000_0000) };

    /// The maximum value for `U40`, which is 1,099,511,627,775 (`0x0000_00ff_ffff_ffff`).
    #[allow(unsafe_code)]
    // SAFETY: This constant is safe because it is guaranteed to be within the valid range of U40.
    pub const MAX: Self = unsafe { Self::new_unchecked(0x0000_00ff_ffff_ffff) };

    /// Create a new `U40` value.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `value` is within the range of [`Self::MIN`]..=[`Self::MAX`].
    #[must_use]
    #[allow(unsafe_code)]
    pub const unsafe fn new_unchecked(value: u64) -> Self {
        Self(value)
    }

    /// Create a new `U40` value if it is within the valid range.
    #[must_use]
    pub const fn new(value: u64) -> Option<Self> {
        if value <= Self::MAX.0 {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Return the underlying `u64` value.
    #[must_use]
    pub const fn into_inner(self) -> u64 {
        self.0
    }
}

impl From<u8> for U40 {
    fn from(value: u8) -> Self {
        #[allow(unsafe_code)]
        // SAFETY: A `u8` value is always within the range of `U40`, so this is safe.
        unsafe {
            Self::new_unchecked(u64::from(value))
        }
    }
}

impl From<u16> for U40 {
    fn from(value: u16) -> Self {
        #[allow(unsafe_code)]
        // SAFETY: A `u16` value is always within the range of `U40`, so this is safe.
        unsafe {
            Self::new_unchecked(u64::from(value))
        }
    }
}

impl From<u32> for U40 {
    fn from(value: u32) -> Self {
        #[allow(unsafe_code)]
        // SAFETY: A `u32` value is always within the range of `U40`, so this is safe.
        unsafe {
            Self::new_unchecked(u64::from(value))
        }
    }
}

impl From<U40> for u64 {
    fn from(value: U40) -> Self {
        value.into_inner()
    }
}

impl TryFrom<u64> for U40 {
    type Error = u64;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(value)
    }
}

impl FromLeStream for U40 {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let bytes = [
            bytes.next()?,
            bytes.next()?,
            bytes.next()?,
            bytes.next()?,
            bytes.next()?,
            0x00,
            0x00,
            0x00,
        ];
        #[allow(unsafe_code)]
        // SAFETY: The bytes are guaranteed to be valid for creating a `U40` value.
        let inner = unsafe { Self::new_unchecked(u64::from_le_bytes(bytes)) };
        Some(inner)
    }
}

impl ToLeStream for U40 {
    type Iter = IntoIter<u8, 5>;

    fn to_le_stream(self) -> Self::Iter {
        let [a, b, c, d, e, _, _, _] = self.0.to_le_bytes();
        [a, b, c, d, e].into_iter()
    }
}
