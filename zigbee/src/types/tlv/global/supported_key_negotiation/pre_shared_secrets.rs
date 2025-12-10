use bitflags::bitflags;
use le_stream::FromLeStream;

/// Pre Shared Secrets bitmask.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct PreSharedSecrets(u8);

bitflags! {
    impl PreSharedSecrets: u8 {
        const SYMMETRIC_AUTHENTICATION_TOKEN = 0b1000_0000;
        const INSTALL_CODE_KEY = 0b0100_0000;
        const PASSCODE_KEY = 0b0010_0000;
        const BASIC_ACCESS_KEY = 0b0001_0000;
        const ADMINISTRATIVE_ACCESS_KEY = 0b0000_1000;
    }
}

impl FromLeStream for PreSharedSecrets {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        u8::from_le_stream(&mut bytes).map(Self::from_bits_retain)
    }
}
