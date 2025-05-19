pub use control::Control;
pub use direction::Direction;
pub use typ::Type;

mod control;
mod direction;
mod typ;

/// A ZCL frame header.

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Header {
    control: Control,
    manufacturer_code: Option<u16>,
    seq: u8,
    command_id: u8,
}

impl Header {
    /// Crate a new header.
    pub const fn new(
        control: Control,
        manufacturer_code: Option<u16>,
        seq: u8,
        command_id: u8,
    ) -> Self {
        // TODO: validate that a set manufacturer code corresponds with the manufacturer specific sub-field being set to 0x01.
        Self {
            control,
            manufacturer_code,
            seq,
            command_id,
        }
    }

    /// Return the control flags.
    pub const fn control(self) -> Control {
        self.control
    }

    /// Return the manufacturer code.
    pub const fn manufacturer_code(self) -> Option<u16> {
        self.manufacturer_code
    }

    /// Return the sequence number.
    pub const fn seq(self) -> u8 {
        self.seq
    }

    /// Return the command ID.
    pub const fn command_id(self) -> u8 {
        self.command_id
    }
}
