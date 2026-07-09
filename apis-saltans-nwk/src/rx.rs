use core::fmt::{self, Display, Formatter, LowerHex, UpperHex};

use apis_saltans_core::IeeeAddress;

/// A payload together with its network-layer source and metadata.
///
/// `Envelope` is generic over the carried payload so higher layers can attach
/// NWK context without tying this crate to APS, ZCL, ZDP, or hardware-specific
/// frame types.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Envelope<T> {
    source: Source,
    metadata: Metadata,
    payload: T,
}

impl<T> Envelope<T> {
    /// Create a new envelope.
    #[must_use]
    pub const fn new(source: Source, metadata: Metadata, payload: T) -> Self {
        Self {
            source,
            metadata,
            payload,
        }
    }

    /// Return the network-layer source.
    #[must_use]
    pub const fn source(&self) -> Source {
        self.source
    }

    /// Return the network-layer metadata.
    #[must_use]
    pub const fn metadata(&self) -> Metadata {
        self.metadata
    }

    /// Return the enclosed payload by reference.
    #[must_use]
    pub const fn payload(&self) -> &T {
        &self.payload
    }

    /// Split the envelope into source, metadata, and payload.
    #[must_use]
    pub fn into_parts(self) -> (Source, Metadata, T) {
        (self.source, self.metadata, self.payload)
    }
}

/// Network-layer source information.
///
/// The source always includes the 16-bit network address. The IEEE address is
/// optional because it may not be known for every incoming frame.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Source {
    node_id: u16,
    ieee_address: Option<IeeeAddress>,
}

impl Source {
    /// Create source information from a network address and optional IEEE address.
    #[must_use]
    pub const fn new(node_id: u16, ieee_address: Option<IeeeAddress>) -> Self {
        Self {
            node_id,
            ieee_address,
        }
    }

    /// Return the 16-bit network address.
    #[must_use]
    pub const fn node_id(&self) -> u16 {
        self.node_id
    }

    /// Return the IEEE address of the source, if known.
    #[must_use]
    pub const fn ieee_address(&self) -> Option<IeeeAddress> {
        self.ieee_address
    }

    /// Split the source into its network address and optional IEEE address.
    #[must_use]
    pub const fn into_parts(self) -> (u16, Option<IeeeAddress>) {
        (self.node_id, self.ieee_address)
    }
}

/// Metadata associated with a received network-layer frame.
///
/// Each field is optional because different backends and frame paths may expose
/// different subsets of NWK metadata.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Metadata {
    last_hop_lqi: Option<u8>,
    last_hop_rssi: Option<i16>,
    binding_index: Option<usize>,
    source_route_overhead: Option<u8>,
}

impl Metadata {
    /// Create network-layer metadata.
    #[must_use]
    pub const fn new(
        last_hop_lqi: Option<u8>,
        last_hop_rssi: Option<i16>,
        binding_index: Option<usize>,
        source_route_overhead: Option<u8>,
    ) -> Self {
        Self {
            last_hop_lqi,
            last_hop_rssi,
            binding_index,
            source_route_overhead,
        }
    }

    /// Return the link quality indicator reported for the last hop.
    #[must_use]
    pub const fn last_hop_lqi(&self) -> Option<u8> {
        self.last_hop_lqi
    }

    /// Return the received signal strength indicator reported for the last hop.
    #[must_use]
    pub const fn last_hop_rssi(&self) -> Option<i16> {
        self.last_hop_rssi
    }

    /// Return the binding table index associated with the frame.
    #[must_use]
    pub const fn binding_index(&self) -> Option<usize> {
        self.binding_index
    }

    /// Return the source-route overhead reported for the frame.
    #[must_use]
    pub const fn source_route_overhead(&self) -> Option<u8> {
        self.source_route_overhead
    }
}

macro_rules! impl_fmt_for_source {
    ($($tr:path),+ $(,)?) => {
        $(
            impl $tr for Source {
                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                    <u16 as $tr>::fmt(&self.node_id, f)?;
                    f.write_str(" (")?;

                    if let Some(ieee_address) = self.ieee_address {
                        <IeeeAddress as $tr>::fmt(&ieee_address, f)?;
                    } else {
                        f.write_str("N/A")?;
                    }

                    f.write_str(")")
                }
            }
        )+
    }
}

impl_fmt_for_source! { Display, UpperHex, LowerHex }
