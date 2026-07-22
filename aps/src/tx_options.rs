use core::fmt::{self, Display};
use core::str::FromStr;

use bitflags::{bitflags, parser};
use le_stream::{FromLeStream, ToLeStream};

/// Transmission options for an APSDE-DATA request.
///
/// Options may be combined to request APS security, acknowledgements, or fragmentation behavior
/// for one application-service data unit. Bits outside the defined Zigbee APSDE-DATA options are
/// reserved and rejected during deserialization.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, ToLeStream)]
#[repr(transparent)]
pub struct TxOptions(u8);

bitflags! {
    impl TxOptions: u8 {
        /// Secure the transmission at the APS layer.
        const SECURITY_ENABLED = 0x01;
        /// Use the network key for APS security.
        const USE_NWK_KEY = 0x02;
        /// Request an APS acknowledgement for the transmission.
        const ACKNOWLEDGED_TRANSMISSION = 0x04;
        /// Permit the APS layer to fragment the application-service data unit.
        const FRAGMENTATION_PERMITTED = 0x08;
        /// Include the sender's extended nonce in the APS security header.
        const INCLUDE_EXTENDED_NONCE = 0x10;
    }
}

impl Display for TxOptions {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        parser::to_writer(self, formatter)
    }
}

impl FromStr for TxOptions {
    type Err = parser::ParseError;

    fn from_str(flags: &str) -> Result<Self, Self::Err> {
        parser::from_str(flags)
    }
}

impl FromLeStream for TxOptions {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        u8::from_le_stream(bytes).and_then(Self::from_bits)
    }
}

#[cfg(test)]
mod tests {
    use le_stream::{FromLeStream, ToLeStream};

    use super::TxOptions;

    const RESERVED_OPTION: u8 = 0x20;

    #[test]
    fn uses_apsde_data_request_bit_assignments() {
        assert_eq!(TxOptions::SECURITY_ENABLED.bits(), 0x01);
        assert_eq!(TxOptions::USE_NWK_KEY.bits(), 0x02);
        assert_eq!(TxOptions::ACKNOWLEDGED_TRANSMISSION.bits(), 0x04);
        assert_eq!(TxOptions::FRAGMENTATION_PERMITTED.bits(), 0x08);
        assert_eq!(TxOptions::INCLUDE_EXTENDED_NONCE.bits(), 0x10);
    }

    #[test]
    fn combined_options_round_trip() {
        let options = TxOptions::SECURITY_ENABLED
            | TxOptions::ACKNOWLEDGED_TRANSMISSION
            | TxOptions::FRAGMENTATION_PERMITTED;
        let bytes: Vec<_> = options.to_le_stream().collect();

        assert_eq!(bytes, [0x0d]);
        assert_eq!(TxOptions::from_le_stream(bytes.into_iter()), Some(options));
    }

    #[test]
    fn rejects_reserved_bits() {
        assert_eq!(
            TxOptions::from_le_stream([RESERVED_OPTION].into_iter()),
            None
        );
    }
}
