use crate::macros::zcl_attribute_newtype;

zcl_attribute_newtype! {
    /// Device power source attribute.
    pub enum PowerSource: Enum8 {
        /// The power source is unknown.
        Unknown = 0x00,
        /// The power source is mains single phase.
        MainsSinglePhase = 0x01,
        /// The power source is mains 3-phase.
        MainsThreePhase = 0x02,
        /// The power source is a battery.
        Battery = 0x03,
        /// The power source is a DC source.
        DcSource = 0x04,
        /// The power source is an emergency mains supply that is constantly powered.
        EmergencyMainsConstantlyPowered = 0x05,
        /// The power source is an emergency mains supply powered through a transfer switch.
        EmergencyMainsAndTransferSwitch = 0x06,
    }
}
