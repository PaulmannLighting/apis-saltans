use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

pub use self::delivery_mode::DeliveryMode;
pub use self::frame_type::FrameType;

mod delivery_mode;
mod frame_type;

/// APS frame control field.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct Control(u8);

bitflags! {
    impl Control: u8 {
        const FRAME_TYPE = 0b1100_0000;
        const DELIVERY_MODE = 0b0011_0000;
        const ACK_FORMAT = 0b0000_1000;
        const SECURITY = 0b0000_0100;
        const ACK_REQUEST = 0b0000_0010;
        const EXTENDED_HEADER = 0b0000_0001;
    }
}

impl Control {
    /// Return the frame type.
    #[must_use]
    pub fn frame_type(self) -> FrameType {
        FrameType::from_u8((self & Self::FRAME_TYPE).bits())
            .unwrap_or_else(|| unreachable!("Frame type covers all possible values."))
    }

    /// Set the frame type.
    pub fn set_frame_type(&mut self, frame_type: FrameType) {
        self.0 = (self.bits() & !Self::FRAME_TYPE.bits()) | ((frame_type as u8) << 6);
    }

    /// Return the delivery mode.
    #[must_use]
    pub fn delivery_mode(self) -> Option<DeliveryMode> {
        DeliveryMode::from_u8((self & Self::DELIVERY_MODE).bits() >> 4)
    }

    /// Set the delivery mode.
    pub fn set_delivery_mode(&mut self, delivery_mode: DeliveryMode) {
        self.0 = (self.bits() & !Self::DELIVERY_MODE.bits()) | ((delivery_mode as u8) << 4);
    }
}
