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
    ///
    /// # Errors
    ///
    /// Returns an error if the direction is not 0x00 or 0x01.
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
            Self::Up(stride) | Self::Down(stride) => stride,
        }
    }
}

#[cfg(feature = "smarthomelib")]
impl<T, U> TryFrom<smarthomelib::Stepping<T>> for Mode<U>
where
    T: TryInto<U>,
{
    type Error = T::Error;

    fn try_from(stepping: smarthomelib::Stepping<T>) -> Result<Self, Self::Error> {
        match stepping {
            smarthomelib::Stepping::Up(step) => Ok(Self::Up(step.try_into()?)),
            smarthomelib::Stepping::Down(step) => Ok(Self::Down(step.try_into()?)),
        }
    }
}
