use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

pub use self::delivery_mode::DeliveryMode;
pub use self::frame_type::FrameType;
use crate::Destination;

mod delivery_mode;
mod frame_type;

/// APS aps control field.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct Control(u8);

bitflags! {
    impl Control: u8 {
        /// Frame type mask.
        const FRAME_TYPE = 0b1100_0000;
        /// Delivery mode mask.
        const DELIVERY_MODE = 0b0011_0000;
        /// Indicate if the aps is a command aps.
        const ACK_FORMAT = 0b0000_1000;
        /// Security provider flag.
        const SECURITY = 0b0000_0100;
        /// Acknowledgment request flag.
        const ACK_REQUEST = 0b0000_0010;
        /// Extended header flag.
        const EXTENDED_HEADER = 0b0000_0001;
    }
}

impl Control {
    /// Return the aps type.
    #[must_use]
    pub fn frame_type(self) -> FrameType {
        FrameType::from_u8(
            (self & Self::FRAME_TYPE).bits() >> Self::FRAME_TYPE.bits().trailing_zeros(),
        )
        .unwrap_or_else(|| unreachable!("Frame type covers all possible values."))
    }

    /// Set the aps type.
    pub const fn set_frame_type(&mut self, frame_type: FrameType) {
        self.0 = (self.bits() & !Self::FRAME_TYPE.bits())
            | ((frame_type as u8) << Self::FRAME_TYPE.bits().trailing_zeros());
    }

    /// Return the delivery mode.
    #[must_use]
    pub fn delivery_mode(self) -> Option<DeliveryMode> {
        DeliveryMode::from_u8(
            (self & Self::DELIVERY_MODE).bits() >> Self::DELIVERY_MODE.bits().trailing_zeros(),
        )
    }

    /// Set the delivery mode.
    pub const fn set_delivery_mode(&mut self, delivery_mode: DeliveryMode) {
        self.0 = (self.bits() & !Self::DELIVERY_MODE.bits())
            | ((delivery_mode as u8) << Self::DELIVERY_MODE.bits().trailing_zeros());
    }

    /// Set the delivery mode based on the destination type.
    pub const fn set_destination(&mut self, destination: Destination) {
        match destination {
            Destination::Unicast(_) => {
                self.set_delivery_mode(DeliveryMode::Unicast);
            }
            Destination::Broadcast(_) => {
                self.set_delivery_mode(DeliveryMode::Broadcast);
            }
            Destination::Group(_) => {
                self.set_delivery_mode(DeliveryMode::Group);
            }
        }
    }
}
