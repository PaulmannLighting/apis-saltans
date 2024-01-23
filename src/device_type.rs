mod green_power;
mod home_automation;
mod light_link;
mod smart_energy;

use crate::profile_type::ProfileType;
pub use green_power::GreenPower;
pub use home_automation::HomeAutomation;
pub use light_link::LightLink;
pub use smart_energy::SmartEnergy;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeviceType {
    HomeAutomation(HomeAutomation),
    GreenPower(GreenPower),
    LightLink(LightLink),
    SmartEnergy(SmartEnergy),
}

impl From<&DeviceType> for ProfileType {
    fn from(device_type: &DeviceType) -> Self {
        match device_type {
            DeviceType::HomeAutomation(_) => ProfileType::HomeAutomation,
            DeviceType::GreenPower(_) => ProfileType::GreenPower,
            DeviceType::LightLink(_) => ProfileType::LightLink,
            DeviceType::SmartEnergy(_) => ProfileType::SmartEnergy,
        }
    }
}

impl From<DeviceType> for ProfileType {
    fn from(device_type: DeviceType) -> Self {
        Self::from(&device_type)
    }
}
