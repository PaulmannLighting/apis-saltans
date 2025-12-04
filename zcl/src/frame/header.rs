use le_stream::{FromLeStream, ToLeStream};
pub use zigbee::Direction;

pub use self::control::Control;
pub use self::typ::Type;

mod control;
mod typ;

/// A ZCL frame header.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
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
        typ: Type,
        direction: Direction,
        disable_client_response: bool,
        manufacturer_code: Option<u16>,
        seq: u8,
        command_id: u8,
    ) -> Self {
        Self {
            control: Control::new(
                typ,
                manufacturer_code.is_some(),
                direction,
                disable_client_response,
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

    /// Return the command ID.
    #[must_use]
    pub const fn command_id(self) -> u8 {
        self.command_id
    }
}
