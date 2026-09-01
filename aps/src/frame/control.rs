use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

pub use self::delivery_mode::DeliveryMode;
pub use self::frame_type::FrameType;
use crate::{Extended, WeakDestination};

mod delivery_mode;
mod frame_type;

/// APS frame control field.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct Control(u8);

bitflags! {
    impl Control: u8 {
        /// Frame type mask.
        const FRAME_TYPE = 0b0000_0011;

        /// Delivery mode mask.
        const DELIVERY_MODE = 0b0000_1100;

        /// Acknowledgment format flag.
        const ACK_FORMAT = 0b0001_0000;

        /// Security provider flag.
        const SECURITY = 0b0010_0000;

        /// Acknowledgment request flag.
        const ACK_REQUEST = 0b0100_0000;

        /// Extended header flag.
        const EXTENDED_HEADER = 0b1000_0000;
    }
}

impl core::fmt::Display for Control {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        bitflags::parser::to_writer(self, formatter)
    }
}

impl core::str::FromStr for Control {
    type Err = bitflags::parser::ParseError;

    fn from_str(flags: &str) -> Result<Self, Self::Err> {
        bitflags::parser::from_str(flags)
    }
}

impl Control {
    /// Return the frame type.
    #[must_use]
    pub fn frame_type(self) -> FrameType {
        FrameType::try_from(
            (self & Self::FRAME_TYPE).bits() >> Self::FRAME_TYPE.bits().trailing_zeros(),
        )
        .unwrap_or_else(|_| unreachable!("Frame type covers all possible values."))
    }

    /// Set the frame type.
    pub const fn set_frame_type(&mut self, frame_type: FrameType) {
        self.0 = (self.bits() & !Self::FRAME_TYPE.bits())
            | ((frame_type as u8) << Self::FRAME_TYPE.bits().trailing_zeros());
    }

    /// Return the delivery mode.
    #[must_use]
    pub fn delivery_mode(self) -> Option<DeliveryMode> {
        DeliveryMode::try_from(
            (self & Self::DELIVERY_MODE).bits() >> Self::DELIVERY_MODE.bits().trailing_zeros(),
        )
        .ok()
    }

    /// Set the delivery mode.
    pub const fn set_delivery_mode(&mut self, delivery_mode: DeliveryMode) {
        self.0 = (self.bits() & !Self::DELIVERY_MODE.bits())
            | ((delivery_mode as u8) << Self::DELIVERY_MODE.bits().trailing_zeros());
    }

    /// Set the delivery mode based on the destination type.
    pub const fn set_destination(&mut self, destination: WeakDestination) {
        match destination {
            WeakDestination::Unicast(_) => {
                self.set_delivery_mode(DeliveryMode::Unicast);
            }
            WeakDestination::Broadcast(_) => {
                self.set_delivery_mode(DeliveryMode::Broadcast);
            }
            WeakDestination::Group(_) => {
                self.set_delivery_mode(DeliveryMode::Group);
            }
        }
    }

    /// Set whether an extended header is present.
    pub fn set_extended_header(&mut self, extended_header_present: bool) {
        if extended_header_present {
            self.insert(Self::EXTENDED_HEADER);
        } else {
            self.remove(Self::EXTENDED_HEADER);
        }
    }

    pub(crate) fn deserialize_extended_header<T>(self, mut bytes: T) -> Result<Option<Extended>, ()>
    where
        T: Iterator<Item = u8>,
    {
        if self.contains(Self::EXTENDED_HEADER) {
            let Some(extended) = Extended::from_le_stream(
                matches!(self.frame_type(), FrameType::Acknowledgment),
                &mut bytes,
            ) else {
                return Err(());
            };

            Ok(Some(extended))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn deserialize_destination<T>(self, mut bytes: T) -> Option<WeakDestination>
    where
        T: Iterator<Item = u8>,
    {
        self.delivery_mode()
            .and_then(|delivery_mode| match delivery_mode {
                DeliveryMode::Unicast => {
                    u8::from_le_stream(&mut bytes).map(WeakDestination::Unicast)
                }
                DeliveryMode::Broadcast => {
                    u8::from_le_stream(&mut bytes).map(WeakDestination::Broadcast)
                }
                DeliveryMode::Group => u16::from_le_stream(&mut bytes).map(WeakDestination::Group),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::{Control, DeliveryMode, FrameType};

    #[test]
    fn bit_assignments_match_wire_format() {
        assert_eq!(Control::FRAME_TYPE.bits(), 0b0000_0011);
        assert_eq!(Control::DELIVERY_MODE.bits(), 0b0000_1100);
        assert_eq!(Control::ACK_FORMAT.bits(), 0b0001_0000);
        assert_eq!(Control::SECURITY.bits(), 0b0010_0000);
        assert_eq!(Control::ACK_REQUEST.bits(), 0b0100_0000);
        assert_eq!(Control::EXTENDED_HEADER.bits(), 0b1000_0000);
    }

    #[test]
    fn fields_are_encoded_at_their_wire_positions() {
        let mut control = Control::SECURITY | Control::ACK_REQUEST | Control::EXTENDED_HEADER;
        control.set_frame_type(FrameType::Command);
        control.set_delivery_mode(DeliveryMode::Broadcast);

        assert_eq!(control.bits(), 0b1110_1001);
        assert_eq!(control.frame_type(), FrameType::Command);
        assert_eq!(control.delivery_mode(), Some(DeliveryMode::Broadcast));
    }
}
