use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

#[derive(Clone, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum ProfileType {
    DeviceProfile = 0x0000,
    IndustrialPlantMonitoring = 0x0101,
    HomeAutomation = 0x0104,
    CommercialBuildingAutomation = 0x0105,
    WirelessSensorNetworks = 0x0106,
    SmartEnergy = 0x0109,
    GreenPower = 0xA1E0,
    LightLink = 0xC05E,
    ManufacturerTelegesis = 0xC059,
    ManufacturerDigi = 0xC105,
}

impl From<ProfileType> for u16 {
    fn from(profile_type: ProfileType) -> Self {
        profile_type
            .to_u16()
            .expect("Could not convert ProfileType to u16")
    }
}
