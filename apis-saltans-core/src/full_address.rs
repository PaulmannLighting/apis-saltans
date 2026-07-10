use crate::{IeeeAddress, ShortId};

/// A Zigbee device address with both long and short address forms.
///
/// Zigbee devices are identified globally by their IEEE address and locally in a
/// network by their NWK short address. `FullAddress` stores both identifiers for
/// code paths that have resolved the complete address pair and need to pass it
/// around as one value.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FullAddress {
    ieee_address: IeeeAddress,
    short_id: ShortId,
}

impl FullAddress {
    /// Create a full address from an IEEE address and NWK short address.
    #[must_use]
    pub const fn new(ieee_address: IeeeAddress, short_id: ShortId) -> Self {
        Self {
            ieee_address,
            short_id,
        }
    }

    /// Return the IEEE address part.
    #[must_use]
    pub const fn ieee_address(&self) -> IeeeAddress {
        self.ieee_address
    }

    /// Return the NWK short-address part.
    #[must_use]
    pub const fn short_id(&self) -> ShortId {
        self.short_id
    }

    /// Split the address into its IEEE and NWK short-address parts.
    #[must_use]
    pub const fn into_parts(self) -> (IeeeAddress, ShortId) {
        (self.ieee_address, self.short_id)
    }
}
