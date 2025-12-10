use std::ops::{Deref, DerefMut};

use le_stream::FromLeStream;

use crate::types::tlv::{EncapsulatedGlobal, Local, Tag, Tlv};

/// Joiner Encapsulation TLV structure.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream)]
pub struct JoinerEncapsulation {
    inner: Vec<Tlv<Local, EncapsulatedGlobal>>,
}

impl Tag for JoinerEncapsulation {
    const TAG: u8 = 72;
}

impl Deref for JoinerEncapsulation {
    type Target = Vec<Tlv<Local, EncapsulatedGlobal>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for JoinerEncapsulation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
