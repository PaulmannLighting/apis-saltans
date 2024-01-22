use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum JoinMethod {
    UseMacAssociation = 0x0,
    UseNwkRejoin = 0x1,
    UseNwkRejoinHaveNwkKey = 0x2,
    UseNwkCommissioning = 0x3,
}

impl From<JoinMethod> for u8 {
    fn from(join_method: JoinMethod) -> Self {
        join_method
            .to_u8()
            .expect("Could not convert JoinMethod to u8")
    }
}
