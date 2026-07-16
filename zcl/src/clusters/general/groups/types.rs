use zb_core::types::{Uint8, Uint16};

const AT_LEAST_ONE_CAPACITY: u8 = Uint8::MAX.into_inner();

/// Remaining capacity of a device's group table.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Capacity {
    /// A concrete number of further groups that may be added.
    Remaining(u8),
    /// At least one further group may be added.
    AtLeastOne,
}

impl Capacity {
    /// Convert a raw ZCL capacity field into a capacity value.
    ///
    /// Returns [`None`] for [`Uint8::NONE`].
    #[must_use]
    pub const fn from_uint8(capacity: Uint8) -> Option<Self> {
        match capacity.as_option() {
            Some(AT_LEAST_ONE_CAPACITY) => Some(Self::AtLeastOne),
            Some(capacity) => Some(Self::Remaining(capacity)),
            None => None,
        }
    }
}

/// A list of group IDs.
pub type GroupList = heapless::Vec<Uint16, { Uint8::MAX.into_inner() as usize }, u8>;
