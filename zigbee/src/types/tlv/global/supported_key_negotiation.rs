use le_stream::FromLeStream;
use macaddr::MacAddr8;

pub use self::key_negotiation_protocols::KeyNegotiationProtocols;
pub use self::pre_shared_secrets::PreSharedSecrets;
use crate::types::tlv::tlv::Tlv;

mod key_negotiation_protocols;
mod pre_shared_secrets;

/// Supported Key Negotiation TLV structure.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream)]
pub struct SupportedKeyNegotiation {
    key_negotiation_protocols: KeyNegotiationProtocols,
    pre_shared_secrets: PreSharedSecrets,
    source_device_eui64: Option<MacAddr8>,
}

impl SupportedKeyNegotiation {
    /// Get the Key Negotiation Protocols.
    #[must_use]
    pub const fn key_negotiation_protocols(&self) -> KeyNegotiationProtocols {
        self.key_negotiation_protocols
    }

    /// Get the Pre Shared Secrets.
    #[must_use]
    pub const fn pre_shared_secrets(&self) -> PreSharedSecrets {
        self.pre_shared_secrets
    }

    /// Get the Source Device EUI-64, if present.
    #[must_use]
    pub const fn source_device_eui64(&self) -> Option<MacAddr8> {
        self.source_device_eui64
    }
}

impl Tlv for SupportedKeyNegotiation {
    const TAG: u8 = 65;
}
