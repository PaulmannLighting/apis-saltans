use crate::command::Scoped;
use crate::frame::HeaderFactory;
use crate::{Command, Header};

/// Marker trait for ZCL native, non-manufacturer specific commands.
pub trait Native {}

#[expect(unsafe_code)]
// SAFETY: We pass in the appropriate scope, direction, disable-default-response flag and command ID.
unsafe impl<T> HeaderFactory for T
where
    T: Command + Scoped + Native,
{
    fn header(&self, seq: u8) -> Header {
        Header::new(
            Self::SCOPE,
            Self::DIRECTION,
            Self::DISABLE_DEFAULT_RESPONSE,
            None,
            seq,
            Self::ID,
        )
    }
}
