use std::fmt::Display;

use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, ToLeStream)]
#[repr(transparent)]
pub struct LeaveReqFlags(u8);

bitflags! {
    impl LeaveReqFlags: u8 {
        /// Rejoin flag.
        const REJOIN = 0b0000_0001;
        /// Remove children flag.
        const REMOVE_CHILDREN = 0b0000_0010;
    }
}

impl Display for LeaveReqFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        let mut names = self.iter_names();

        if let Some((name, flag)) = names.next() {
            write!(f, "{name} ({flag:#04X})")?;

            for (name, flag) in names {
                write!(f, "{name} ({flag:#04X})")?;
            }
        }

        write!(f, "]")
    }
}

impl FromLeStream for LeaveReqFlags {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        u8::from_le_stream(bytes).map(Self::from_bits_truncate)
    }
}
