use std::ops::{Deref, DerefMut};

use le_stream::FromLeStream;

pub use self::encapsulated_global::EncapsulatedGlobal;
use crate::types::tlv::{Tag, Tlv};

mod encapsulated_global;

/// Joiner Encapsulation TLV structure.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream)]
pub struct JoinerEncapsulation {
    // TODO: replace () with appropriate type for local TLVs.
    inner: Vec<Tlv<(), EncapsulatedGlobal>>,
}

impl Tag for JoinerEncapsulation {
    const TAG: u8 = 72;
}

impl Deref for JoinerEncapsulation {
    type Target = Vec<Tlv<(), EncapsulatedGlobal>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for JoinerEncapsulation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
