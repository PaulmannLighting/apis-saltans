use le_stream::{FromLeStream, ToLeStream};

/// Options struct for cluster commands.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, ToLeStream)]
pub struct Options {
    mask: u8,
    r#override: u8,
}

impl Options {
    /// Create a new `Options` instance.
    #[must_use]
    pub const fn new(mask: u8, r#override: u8) -> Self {
        Self { mask, r#override }
    }

    /// Get the options mask.
    #[must_use]
    pub const fn mask(self) -> u8 {
        self.mask
    }

    /// Get the options override.
    #[must_use]
    pub const fn r#override(self) -> u8 {
        self.r#override
    }
}

impl FromLeStream for Options {
    /// Create an `Options` instance from a little-endian byte stream.
    ///
    /// This is infallible, by defaulting to `0` on insufficient bytes.
    /// This ensures that deserialization also succeeds for legacy devices not sending these fields.
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        Some(Self::new(
            u8::from_le_stream(&mut bytes).unwrap_or_default(),
            u8::from_le_stream(bytes).unwrap_or_default(),
        ))
    }
}
