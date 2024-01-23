use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

#[derive(Clone, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum SmartEnergy {
    RangeExtender = 0x0008,
    EnergyServiceInterface = 0x0500,
    MeteringDevice = 0x0501,
    InHomeDisplay = 0x0502,
    ProgrammableCommunicatingThermostat = 0x0503,
    LoadControlDevice = 0x0504,
    SmartAppliance = 0x0505,
    PrepaymentTerminal = 0x0506,
    PhysicalDevice = 0x0507,
}

impl From<SmartEnergy> for u16 {
    fn from(smart_energy: SmartEnergy) -> Self {
        smart_energy
            .to_u16()
            .expect("Could not convert SmartEnergy to u16")
    }
}
