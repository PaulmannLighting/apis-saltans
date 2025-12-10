use bitflags::bitflags;
use le_stream::FromLeStream;

/// Key Negotiation Protocols bitmask.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct KeyNegotiationProtocols(u8);

bitflags! {
    impl KeyNegotiationProtocols: u8 {
        /// Static Key Request (Zigbee 3.0 Mechanism, TCLK procedure).
        const STATIC_KEY_REQUEST = 0b1000_0000;
        /// SPEKE using Curve25519 with Hash AES-MMO-128.
        const SPEKE_USING_CURVE25519_WITH_HASH_AES_MMO_128 = 0b0100_0000;
        /// SPEKE using Curve25519 with Hash SHA-256.
        const SPEKE_USING_CURVE25519_WITH_HASH_SHA256 = 0b0010_0000;
    }
}

impl FromLeStream for KeyNegotiationProtocols {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        u8::from_le_stream(bytes).map(Self::from_bits_retain)
    }
}
