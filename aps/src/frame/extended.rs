use le_stream::{FromLeStream, ToLeStream};

pub use self::control::Control;
pub use self::fragmentation::Fragmentation;

mod control;
mod fragmentation;

/// Extended header.
/// TODO: Implement fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, ToLeStream)]
pub struct Extended {
    control: Control,
    block_number: Option<u8>,
    bit_field: Option<u8>,
}

impl Extended {
    /// Return the control field.
    #[must_use]
    pub const fn control(&self) -> Control {
        self.control
    }

    /// Return the block number.
    #[must_use]
    pub const fn block_number(&self) -> Option<u8> {
        self.block_number
    }

    /// Return the bit field.
    #[must_use]
    pub const fn bit_field(&self) -> Option<u8> {
        self.bit_field
    }
}

impl Extended {
    pub(crate) fn from_le_stream<T>(is_ack: bool, mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let control = Control::from_le_stream(&mut bytes)?;

        let Some(fragmentation) = control.fragmentation() else {
            return Some(Self {
                control,
                block_number: None,
                bit_field: None,
            });
        };

        if fragmentation == Fragmentation::NotFragmented {
            return Some(Self {
                control,
                block_number: None,
                bit_field: None,
            });
        }

        let block_number = u8::from_le_stream(&mut bytes)?;

        if is_ack {
            return Some(Self {
                control,
                block_number: Some(block_number),
                bit_field: Some(u8::from_le_stream(&mut bytes)?),
            });
        }

        Some(Self {
            control,
            block_number: Some(block_number),
            bit_field: None,
        })
    }
}
