use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

#[derive(Clone, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum LightLink {
    OnOffLight = 0x0000,
    OnOffPlugInUnit = 0x0010,
    DimmableLight = 0x0100,
    DimmablePlugInUnit = 0x0110,
    ColorLight = 0x0200,
    ExtendedColorLight = 0x0210,
    ColorTemperatureLight = 0x0220,
    ColorController = 0x0800,
    ColorSceneController = 0x0810,
    NonColorController = 0x0820,
    NonColorSceneController = 0x0830,
    ControlBridge = 0x0840,
    OnOffSensor = 0x0850,
}

impl From<LightLink> for u16 {
    fn from(light_link: LightLink) -> Self {
        light_link
            .to_u16()
            .expect("Could not convert LightLink to u16")
    }
}
