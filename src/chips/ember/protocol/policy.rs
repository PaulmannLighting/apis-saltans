use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum Policy {
    TrustCenter = 0x00,
    BindingModification = 0x01,
    UnicastReplies = 0x02,
    PollHandler = 0x03,
    MessageContentsInCallback = 0x04,
    TcKeyRequest = 0x05,
    AppKeyRequest = 0x06,
    PacketValidateLibrary = 0x07,
    Zll = 0x08,
    TcRejoinsUsingWellKnownKey = 0x09,
}

impl From<Policy> for u8 {
    fn from(policy: Policy) -> Self {
        policy.to_u8().expect("Could not convert Policy to u8")
    }
}
