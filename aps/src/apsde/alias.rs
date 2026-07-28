use super::NetworkAddress;

/// Optional NWK-layer alias parameters for an `APSDE-DATA.request`.
///
/// Grouping the alias source and sequence number prevents a request from
/// carrying only one of the two values required when aliasing is enabled.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Alias {
    /// Do not use a NWK source alias.
    #[default]
    None,

    /// Use the supplied NWK source address and sequence number.
    Use {
        /// Non-broadcast NWK source address.
        source: NetworkAddress,
        /// NWK sequence number to use for the aliased transmission.
        sequence_number: u8,
    },
}

impl Alias {
    /// Return whether this request uses a NWK source alias.
    #[must_use]
    pub const fn is_used(self) -> bool {
        matches!(self, Self::Use { .. })
    }
}
