use bitflags::bitflags;

/// The `Update` flags for the Color Loop Set command in the Lighting cluster.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Update(u8);

bitflags! {
    impl Update: u8 {
        /// Flag whether to adhere to the action field.
        const UPDATE_ACTION = 0b0000_0001;
        /// Flag whether to update the `ColorLoopDirection` attribute.
        const UPDATE_DIRECTION = 0b0000_0010;
        /// Flag whether to update the `ColorLoopTime` attribute.
        const UPDATE_TIME = 0b0000_0100;
        /// Flag whether to update the `ColorLoopStartEnhancedHue` attribute.
        const UPDATE_START_HUE = 0b0000_1000;
    }
}
