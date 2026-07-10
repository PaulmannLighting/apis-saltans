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

impl_fmt_pair!(
    FullAddress,
    IeeeAddress,
    ShortId,
    |value| (value.ieee_address, value.short_id),
    "/"
);

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::format;

    use crate::{FullAddress, IeeeAddress, ShortId, short_id};

    const IEEE_ADDRESS: IeeeAddress = IeeeAddress::new(1, 2, 3, 4, 5, 6, 7, 8);
    const SHORT_ID: ShortId = ShortId::Device(short_id::Device(0x1234));
    const ADDRESS: FullAddress = FullAddress::new(IEEE_ADDRESS, SHORT_ID);

    #[test]
    fn display_formats_both_parts() {
        assert_eq!(format!("{ADDRESS}"), format!("{IEEE_ADDRESS}/{SHORT_ID}"));
    }

    #[test]
    fn lower_hex_formats_both_parts() {
        assert_eq!(
            format!("{ADDRESS:x}"),
            format!("{IEEE_ADDRESS:x}/{SHORT_ID:x}")
        );
    }

    #[test]
    fn upper_hex_formats_both_parts() {
        assert_eq!(
            format!("{ADDRESS:X}"),
            format!("{IEEE_ADDRESS:X}/{SHORT_ID:X}")
        );
    }
}
