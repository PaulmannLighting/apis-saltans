use le_stream::{FromLeStream, ToLeStream};
pub use zigbee::Direction;

pub use self::control::Control;
pub use self::scope::Scope;

mod control;
mod scope;

/// A ZCL frame header.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ToLeStream)]
pub struct Header {
    control: Control,
    manufacturer_code: Option<u16>,
    seq: u8,
    command_id: u8,
}

impl Header {
    /// Crate a new header.
    #[must_use]
    pub fn new(
        typ: Scope,
        direction: Direction,
        disable_default_response: bool,
        manufacturer_code: Option<u16>,
        seq: u8,
        command_id: u8,
    ) -> Self {
        Self {
            control: Control::new(
                typ,
                manufacturer_code.is_some(),
                direction,
                disable_default_response,
            ),
            manufacturer_code,
            seq,
            command_id,
        }
    }

    /// Return the control flags.
    #[must_use]
    pub const fn control(self) -> Control {
        self.control
    }

    /// Return the manufacturer code.
    #[must_use]
    pub const fn manufacturer_code(self) -> Option<u16> {
        self.manufacturer_code
    }

    /// Return the sequence number.
    #[must_use]
    pub const fn seq(self) -> u8 {
        self.seq
    }

    /// Set the sequence number.
    pub const fn set_seq(&mut self, seq: u8) {
        self.seq = seq;
    }

    /// Return the command ID.
    #[must_use]
    pub const fn command_id(self) -> u8 {
        self.command_id
    }
}

impl FromLeStream for Header {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let control = Control::from_le_stream(&mut bytes)?;

        let manufacturer_code = if control.is_manufacturer_specific() {
            Some(u16::from_le_stream(&mut bytes)?)
        } else {
            None
        };

        let seq = bytes.next()?;
        let command_id = bytes.next()?;

        Some(Self {
            control,
            manufacturer_code,
            seq,
            command_id,
        })
    }
}
