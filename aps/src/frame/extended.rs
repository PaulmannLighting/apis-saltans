use le_stream::{FromLeStream, ToLeStream};

pub use self::control::Control;
pub use self::fragmentation::Fragmentation;

mod control;
mod fragmentation;

/// Extended header.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, ToLeStream)]
pub struct Extended {
    control: Control,
    block_number: Option<u8>,
    bit_field: Option<u8>,
}

impl Extended {
    /// Create a new `Extended` header for an initial fragment.
    #[must_use]
    pub const fn first_fragment(blocks: u8) -> Self {
        Self {
            control: Control::FIRST_FRAGMENT,
            block_number: Some(blocks),
            bit_field: None,
        }
    }

    /// Create a new `Extended` header for a follow-up fragment.
    #[must_use]
    pub const fn followup_fragment(index: u8) -> Self {
        Self {
            control: Control::FOLLOWUP_FRAGMENT,
            block_number: Some(index),
            bit_field: None,
        }
    }

    /// Create an extended header for the given fragmentation state.
    ///
    /// Returns an empty extended header for [`Fragmentation::None`].
    #[must_use]
    pub fn fragment(fragmentation: Fragmentation) -> Self {
        match fragmentation {
            Fragmentation::None => Self::default(),
            Fragmentation::First { blocks } => Self::first_fragment(blocks),
            Fragmentation::Followup { index } => Self::followup_fragment(index),
        }
    }

    /// Return the control field.
    #[must_use]
    pub const fn control(self) -> Control {
        self.control
    }

    /// Return the block number.
    #[must_use]
    pub const fn block_number(self) -> Option<u8> {
        self.block_number
    }

    /// Return the bit field.
    #[must_use]
    pub const fn bit_field(self) -> Option<u8> {
        self.bit_field
    }

    pub(crate) fn from_le_stream<T>(is_ack: bool, mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let control = Control::from_le_stream(&mut bytes)?;

        let block_number = if control.intersects(Control::FRAGMENTATION) {
            Some(u8::from_le_stream(&mut bytes)?)
        } else {
            None
        };

        let bit_field = if is_ack {
            Some(u8::from_le_stream(&mut bytes)?)
        } else {
            None
        };

        Some(Self {
            control,
            block_number,
            bit_field,
        })
    }
}
