//! ZCL frames awaiting assignment of a transaction sequence number.

use bytes::Bytes;
use le_stream::ToLeStream;

use super::{Control, Direction, Frame, Header, Scope};
use crate::{Command, Directed, Scoped};

/// A ZCL header awaiting assignment of its transaction sequence number.
///
/// This type has no wire representation. Consume it with [`Self::into_header`] when the sender
/// assigns the transaction sequence number.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UnsequencedHeader {
    control: Control,
    manufacturer_code: Option<u16>,
    command_id: u8,
}

impl UnsequencedHeader {
    /// Create an unsequenced ZCL header.
    #[must_use]
    pub fn new(
        scope: Scope,
        direction: Direction,
        disable_default_response: bool,
        manufacturer_code: Option<u16>,
        command_id: u8,
    ) -> Self {
        Self {
            control: Control::new(
                scope,
                manufacturer_code.is_some(),
                direction,
                disable_default_response,
            ),
            manufacturer_code,
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

    /// Set whether the default response is disabled.
    pub fn set_disable_default_response(&mut self, disabled: bool) {
        self.control
            .set(Control::DISABLE_DEFAULT_RESPONSE, disabled);
    }

    /// Return the command ID.
    #[must_use]
    pub const fn command_id(self) -> u8 {
        self.command_id
    }

    /// Assign a transaction sequence number and create a complete ZCL header.
    #[must_use]
    pub const fn into_header(self, sequence_number: u8) -> Header {
        Header::from_parts(
            self.control,
            self.manufacturer_code,
            sequence_number,
            self.command_id,
        )
    }
}

/// A ZCL frame awaiting assignment of its transaction sequence number.
///
/// This type deliberately does not implement little-endian stream serialization. Consume it with
/// [`Self::into_frame`] before encoding it for transmission.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UnsequencedFrame<T> {
    header: UnsequencedHeader,
    payload: T,
}

impl<T> UnsequencedFrame<T> {
    /// Create an unsequenced ZCL frame from its header and payload.
    #[must_use]
    pub const fn new(header: UnsequencedHeader, payload: T) -> Self {
        Self { header, payload }
    }

    /// Return the unsequenced header.
    #[must_use]
    pub const fn header(&self) -> UnsequencedHeader {
        self.header
    }

    /// Return a reference to the payload.
    #[must_use]
    pub const fn payload(&self) -> &T {
        &self.payload
    }

    /// Consume the frame and return its payload.
    #[must_use]
    pub fn into_payload(self) -> T {
        self.payload
    }

    /// Consume the frame and return its header and payload.
    #[must_use]
    pub fn into_parts(self) -> (UnsequencedHeader, T) {
        (self.header, self.payload)
    }

    /// Override whether the frame disables the default response.
    #[must_use]
    pub fn with_disable_default_response(mut self, disabled: bool) -> Self {
        self.header.set_disable_default_response(disabled);
        self
    }

    /// Transform the payload while preserving the unsequenced ZCL header.
    #[must_use]
    pub fn map_payload<U, F>(self, map: F) -> UnsequencedFrame<U>
    where
        F: FnOnce(T) -> U,
    {
        UnsequencedFrame {
            header: self.header,
            payload: map(self.payload),
        }
    }

    /// Assign a transaction sequence number and create a complete ZCL frame.
    #[must_use]
    pub fn into_frame(self, sequence_number: u8) -> Frame<T> {
        Frame::new(self.header.into_header(sequence_number), self.payload)
    }
}

impl UnsequencedFrame<Bytes> {
    /// Convert a typed command into an unsequenced ZCL frame.
    #[must_use]
    pub fn from_command<T>(command: T) -> Self
    where
        T: Command + Directed + Scoped + ToLeStream,
    {
        Self::new(
            UnsequencedHeader::new(
                T::SCOPE,
                T::DIRECTION,
                T::DISABLE_DEFAULT_RESPONSE,
                T::MANUFACTURER_CODE,
                <T as Command>::ID,
            ),
            command.to_le_stream().collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use le_stream::FromLeStream;

    use super::{Direction, Scope, UnsequencedFrame, UnsequencedHeader};
    use crate::Command;
    use crate::on_off::On;

    const COMMAND_ID: u8 = 1;
    const MANUFACTURER_CODE: u16 = 0x1234;
    const SEQUENCE_NUMBER: u8 = 42;

    #[test]
    fn assigns_sequence_number_when_consumed() {
        let frame = UnsequencedFrame::new(
            UnsequencedHeader::new(
                Scope::ClusterSpecific,
                Direction::ServerToClient,
                true,
                Some(MANUFACTURER_CODE),
                COMMAND_ID,
            ),
            Bytes::new(),
        )
        .into_frame(SEQUENCE_NUMBER);

        assert_eq!(frame.header().seq(), SEQUENCE_NUMBER);
        assert_eq!(frame.header().manufacturer_code(), Some(MANUFACTURER_CODE));
        assert_eq!(frame.header().command_id(), COMMAND_ID);
        assert_eq!(frame.header().control().typ(), Ok(Scope::ClusterSpecific));
        assert_eq!(
            frame.header().control().direction(),
            Direction::ServerToClient
        );
        assert!(frame.header().control().disable_default_response());
    }

    #[test]
    fn overrides_default_response_before_sequence_assignment() {
        let frame = UnsequencedFrame::new(
            UnsequencedHeader::new(
                Scope::Global,
                Direction::ClientToServer,
                true,
                None,
                COMMAND_ID,
            ),
            Bytes::new(),
        )
        .with_disable_default_response(false)
        .into_frame(SEQUENCE_NUMBER);

        assert!(!frame.header().control().disable_default_response());
    }

    #[test]
    fn constructs_an_unsequenced_frame_from_a_typed_command() {
        let frame = UnsequencedFrame::from_command(On);

        assert_eq!(frame.header().command_id(), <On as Command>::ID);
        assert_eq!(
            On::from_le_stream(frame.payload().clone().into_iter()),
            Some(On)
        );
    }
}
