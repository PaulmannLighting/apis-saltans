use std::iter::Chain;

use le_stream::{FromLeStream, ToLeStream};
use macaddr::MacAddr8;

pub use self::key_negotiation_protocols::KeyNegotiationProtocols;
pub use self::pre_shared_secrets::PreSharedSecrets;
use crate::types::tlv::Tag;

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
    /// Create a new `SupportedKeyNegotiation`.
    #[must_use]
    pub const fn new(
        key_negotiation_protocols: KeyNegotiationProtocols,
        pre_shared_secrets: PreSharedSecrets,
        source_device_eui64: Option<MacAddr8>,
    ) -> Self {
        Self {
            key_negotiation_protocols,
            pre_shared_secrets,
            source_device_eui64,
        }
    }

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

impl Tag for SupportedKeyNegotiation {
    const TAG: u8 = 65;

    fn size(&self) -> usize {
        let mut size = 1 + 1;

        if self.source_device_eui64.is_some() {
            size += 8;
        }

        size
    }
}

impl ToLeStream for SupportedKeyNegotiation {
    type Iter = Chain<
        Chain<
            Chain<
                Chain<<u8 as ToLeStream>::Iter, <u8 as ToLeStream>::Iter>,
                <KeyNegotiationProtocols as ToLeStream>::Iter,
            >,
            <PreSharedSecrets as ToLeStream>::Iter,
        >,
        <Option<MacAddr8> as ToLeStream>::Iter,
    >;

    fn to_le_stream(self) -> Self::Iter {
        Self::TAG
            .to_le_stream()
            .chain(self.serialized_size().to_le_stream())
            .chain(self.key_negotiation_protocols.to_le_stream())
            .chain(self.pre_shared_secrets.to_le_stream())
            .chain(self.source_device_eui64.to_le_stream())
    }
}
