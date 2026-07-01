use repr_discriminant::ReprDiscriminant;

/// Move mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, ReprDiscriminant)]
#[repr(u8)]
pub enum Mode<T> {
    /// Move up.
    Up(T) = 0x00,

    /// Move down.
    Down(T) = 0x01,
}

impl<T> Mode<T> {
    /// Create a new move mode.
    pub const fn new(direction: u8, stride: T) -> Result<Self, T> {
        match direction {
            0x00 => Ok(Self::Up(stride)),
            0x01 => Ok(Self::Down(stride)),
            _ => Err(stride),
        }
    }

    /// Get the stride.
    #[must_use]
    pub fn into_stride(self) -> T {
        match self {
            Self::Up(value) => value,
            Self::Down(value) => value,
        }
    }
}
