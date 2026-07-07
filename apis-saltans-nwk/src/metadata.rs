#[cfg_attr(
    feature = "le-stream",
    derive(le_stream::FromLeStream, le_stream::ToLeStream)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Metadata {
    last_hop_lqi: Option<u8>,
    last_hop_rssi: Option<i16>,
    binding_index: Option<usize>,
    source_route_overhead: Option<u8>,
}

impl Metadata {
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

    #[must_use]
    pub const fn last_hop_lqi(&self) -> Option<u8> {
        self.last_hop_lqi
    }

    #[must_use]
    pub const fn last_hop_rssi(&self) -> Option<i16> {
        self.last_hop_rssi
    }

    #[must_use]
    pub const fn binding_index(&self) -> Option<usize> {
        self.binding_index
    }

    #[must_use]
    pub const fn source_route_overhead(&self) -> Option<u8> {
        self.source_route_overhead
    }
}
