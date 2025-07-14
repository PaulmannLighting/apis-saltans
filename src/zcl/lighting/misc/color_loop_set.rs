use bitflags::bitflags;
use num_derive::FromPrimitive;

/// The `Update` flags for the Color Loop Set command in the Lighting cluster.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Update(u8);

bitflags! {
    impl Update: u8 {
        /// Flag whether to adhere to the action field.
        const UPDATE_ACTION = 0b1000_0000;
        /// Flag whether to update the `ColorLoopDirection` attribute.
        const UPDATE_DIRECTION = 0b0100_0000;
        /// Flag whether to update the `ColorLoopTime` attribute.
        const UPDATE_TIME = 0b0010_0000;
        /// Flag whether to update the `ColorLoopStartEnhancedHue` attribute.
        const UPDATE_START_HUE = 0b0001_0000;
    }
}

/// Available color loop set actions.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Action {
    Deactivate = 0x00,
    ActivateFromColorLoopStartEnhancedHue = 0x01,
    ActivateFromEnhancedCurrentHue = 0x02,
}

/// The direction of the color loop.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Direction {
    Decrement = 0x00,
    Increment = 0x01,
}
