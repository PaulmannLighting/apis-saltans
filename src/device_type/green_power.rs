use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

#[derive(Clone, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum GreenPower {
    Proxy = 0x0060,
    ProxyBasic = 0x0061,
    TargetPlus = 0x0062,
    Target = 0x0063,
    CommissioningTool = 0x0064,
    Combo = 0x0065,
    ComboBasic = 0x0066,
}

impl From<GreenPower> for u16 {
    fn from(green_power: GreenPower) -> Self {
        green_power
            .to_u16()
            .expect("Could not convert GreenPower to u16")
    }
}
