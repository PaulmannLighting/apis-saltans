use std::ops::{Deref, DerefMut};

use le_stream::FromLeStream;

use crate::types::tlv::{EncapsulatedGlobal, Local, Tag, Tlv};

/// Beacon Appendix Encapsulation TLV structure.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream)]
pub struct BeaconAppendixEncapsulation {
    inner: Vec<Tlv<Local, EncapsulatedGlobal>>,
}

impl Tag for BeaconAppendixEncapsulation {
    const TAG: u8 = 73;
}

impl Deref for BeaconAppendixEncapsulation {
    type Target = Vec<Tlv<Local, EncapsulatedGlobal>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for BeaconAppendixEncapsulation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
