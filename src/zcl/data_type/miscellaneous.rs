use macaddr::MacAddr8;

/// Miscellaneous data types used in Zigbee clusters.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Miscellaneous {
    IeeeAddress(MacAddr8),
    SecurityKey([u8; 16]),
    Opaque(Vec<u8>),
}
