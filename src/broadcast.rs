use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

#[derive(Clone, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Destination {
    AllDevices = 0xFFFF,
    RxOn = 0xFFFD,
    RoutersAndCoord = 0xFFFC,
    LowPowerRouters = 0xFFFB,
    ReservedFffe = 0xFFFE,
    ReservedFffa = 0xFFFA,
    ReservedFff9 = 0xFFF9,
    ReservedFff8 = 0xFFF8,
}

impl From<Destination> for u16 {
    fn from(destination: Destination) -> Self {
        destination
            .to_u16()
            .expect("Could not convert Destination to u16")
    }
}
