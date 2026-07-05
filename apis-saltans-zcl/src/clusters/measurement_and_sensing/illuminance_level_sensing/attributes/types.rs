use crate::macros::zcl_attribute_newtype;

zcl_attribute_newtype! {
    /// Whether the illuminance is on target.
    pub enum LevelStatus: Enum8 {
        /// Illuminance on target.
        OnTarget = 0x00,
        /// Illuminance below target.
        BelowTarget = 0x01,
        /// Illuminance above target.
        AboveTarget = 0x02,
    }
}
