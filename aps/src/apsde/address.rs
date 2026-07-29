use core::fmt::{self, Display, Formatter, LowerHex, UpperHex};

use num_enum::{IntoPrimitive, TryFromPrimitive};
use zb_core::{Endpoint, GroupId, IeeeAddress, ShortId, short_id};

const MAX_NETWORK_ADDRESS: u16 = 0xfff7;
const MIN_BROADCAST_ADDRESS: u16 = 0xfffc;

/// Address-mode values used by APSDE-DATA primitives.
///
/// Individual primitive address types restrict which of these modes are
/// valid in their context.
#[derive(
    Clone, Copy, Debug, Eq, Hash, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive,
)]
#[num_enum(error_type(name = u8, constructor = core::convert::identity))]
#[repr(u8)]
pub enum AddressMode {
    /// Resolve destinations through the local binding table.
    Bound = 0x00,

    /// Address a 16-bit APS group.
    Group = 0x01,

    /// Address a 16-bit NWK address and endpoint.
    Network = 0x02,

    /// Address a 64-bit IEEE address and endpoint.
    Extended = 0x03,

    /// Address a 64-bit IEEE address without an endpoint.
    ExtendedWithoutEndpoint = 0x04,
}

/// A non-broadcast 16-bit Zigbee NWK address.
///
/// This range contains the coordinator and allocated device addresses. It
/// excludes reserved, invalid, and broadcast short-address values.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NetworkAddress(u16);

/// APSDE multicast broadcast selector.
///
/// APSDE-DATA requests with group addressing carry a separate NWK broadcast
/// address in the inclusive range `0xfffc..=0xffff`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BroadcastAddress(u16);

/// An endpoint that addresses one local application or the ZDO data service.
///
/// The APS broadcast endpoint is deliberately excluded.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IndividualEndpoint(Endpoint);

/// Destination fields accepted by an `APSDE-DATA.request`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RequestDestination {
    /// Resolve one or more destinations through the binding table.
    Bound,

    /// Send to an APS group through the selected NWK broadcast receiver set.
    Group {
        /// APS group identifier.
        address: GroupId,
        /// NWK broadcast address used to transport the group message.
        broadcast_address: BroadcastAddress,
    },

    /// Send to a 16-bit NWK broadcast address and endpoint.
    Broadcast {
        /// NWK broadcast receiver set.
        address: short_id::Broadcast,
        /// Destination endpoint, including the APS broadcast endpoint when required.
        endpoint: Endpoint,
    },

    /// Send to a 16-bit NWK address and endpoint.
    Network {
        /// Destination NWK address.
        address: NetworkAddress,
        /// Destination endpoint, including the APS broadcast endpoint when required.
        endpoint: Endpoint,
    },

    /// Resolve and send to a 64-bit IEEE address and endpoint.
    Extended {
        /// Destination IEEE address.
        address: IeeeAddress,
        /// Destination endpoint, including the APS broadcast endpoint when required.
        endpoint: Endpoint,
    },
}

/// Destination fields reported by an `APSDE-DATA.confirm`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Destination {
    /// The request used binding-table destination resolution.
    Bound,

    /// The request addressed an APS group.
    Group(GroupId),

    /// The request addressed a 16-bit NWK address and endpoint.
    Network {
        /// Destination NWK address.
        address: NetworkAddress,
        /// Destination endpoint, including the APS broadcast endpoint when requested.
        endpoint: Endpoint,
    },

    /// The request addressed a 64-bit IEEE address and endpoint.
    Extended {
        /// Destination IEEE address.
        address: IeeeAddress,
        /// Destination endpoint, including the APS broadcast endpoint when requested.
        endpoint: Endpoint,
    },
}

/// Destination fields reported by an `APSDE-DATA.indication`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ReceivedDestination {
    /// The received ASDU was addressed to an APS group.
    Group(GroupId),

    /// The received ASDU was addressed to a 16-bit NWK address and endpoint.
    Network {
        /// Destination NWK address.
        address: NetworkAddress,
        /// Local target endpoint.
        endpoint: IndividualEndpoint,
    },

    /// The received ASDU was addressed to a 64-bit IEEE address and endpoint.
    Extended {
        /// Destination IEEE address.
        address: IeeeAddress,
        /// Local target endpoint.
        endpoint: IndividualEndpoint,
    },

    /// The received ASDU carried a 64-bit IEEE destination without an endpoint.
    ExtendedWithoutEndpoint(IeeeAddress),
}

/// Source fields reported by an `APSDE-DATA.indication`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Source {
    /// The ASDU originated at a 16-bit NWK address and endpoint.
    Network {
        /// Source NWK address.
        address: NetworkAddress,
        /// Source endpoint.
        endpoint: IndividualEndpoint,
    },

    /// The ASDU originated at a 64-bit IEEE address and endpoint.
    Extended {
        /// Source IEEE address.
        address: IeeeAddress,
        /// Source endpoint.
        endpoint: IndividualEndpoint,
    },

    /// The ASDU carried a 64-bit IEEE source without an endpoint.
    ExtendedWithoutEndpoint(IeeeAddress),
}

impl NetworkAddress {
    /// Create a non-broadcast NWK address.
    #[must_use]
    pub const fn new(address: u16) -> Option<Self> {
        if address <= MAX_NETWORK_ADDRESS {
            Some(Self(address))
        } else {
            None
        }
    }

    /// Return the raw 16-bit address.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self.0
    }
}

impl BroadcastAddress {
    /// Create an APSDE multicast broadcast selector.
    #[must_use]
    pub const fn new(address: u16) -> Option<Self> {
        if address >= MIN_BROADCAST_ADDRESS {
            Some(Self(address))
        } else {
            None
        }
    }

    /// Return the raw 16-bit broadcast selector.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self.0
    }
}

impl IndividualEndpoint {
    /// Create an individual endpoint.
    #[must_use]
    pub const fn new(endpoint: Endpoint) -> Option<Self> {
        match endpoint {
            Endpoint::Broadcast => None,
            Endpoint::Data | Endpoint::Application(_) => Some(Self(endpoint)),
        }
    }

    /// Return the underlying Zigbee endpoint.
    #[must_use]
    pub const fn get(self) -> Endpoint {
        self.0
    }
}

impl RequestDestination {
    /// Return the APSDE destination addressing mode.
    #[must_use]
    pub const fn mode(self) -> AddressMode {
        match self {
            Self::Bound => AddressMode::Bound,
            Self::Group { .. } => AddressMode::Group,
            Self::Broadcast { .. } | Self::Network { .. } => AddressMode::Network,
            Self::Extended { .. } => AddressMode::Extended,
        }
    }
}

impl Destination {
    /// Return the APSDE destination addressing mode.
    #[must_use]
    pub const fn mode(self) -> AddressMode {
        match self {
            Self::Bound => AddressMode::Bound,
            Self::Group(_) => AddressMode::Group,
            Self::Network { .. } => AddressMode::Network,
            Self::Extended { .. } => AddressMode::Extended,
        }
    }
}

impl ReceivedDestination {
    /// Return the received destination addressing mode.
    #[must_use]
    pub const fn mode(self) -> AddressMode {
        match self {
            Self::Group(_) => AddressMode::Group,
            Self::Network { .. } => AddressMode::Network,
            Self::Extended { .. } => AddressMode::Extended,
            Self::ExtendedWithoutEndpoint(_) => AddressMode::ExtendedWithoutEndpoint,
        }
    }
}

impl Source {
    /// Return the received source addressing mode.
    #[must_use]
    pub const fn mode(self) -> AddressMode {
        match self {
            Self::Network { .. } => AddressMode::Network,
            Self::Extended { .. } => AddressMode::Extended,
            Self::ExtendedWithoutEndpoint(_) => AddressMode::ExtendedWithoutEndpoint,
        }
    }
}

macro_rules! impl_address_format {
    ($type:ty) => {
        impl Display for $type {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
                Display::fmt(&self.0, formatter)
            }
        }

        impl LowerHex for $type {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
                LowerHex::fmt(&self.0, formatter)
            }
        }

        impl UpperHex for $type {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
                UpperHex::fmt(&self.0, formatter)
            }
        }

        impl From<$type> for u16 {
            fn from(address: $type) -> Self {
                address.0
            }
        }

        impl TryFrom<u16> for $type {
            type Error = u16;

            fn try_from(address: u16) -> Result<Self, Self::Error> {
                Self::new(address).ok_or(address)
            }
        }
    };
}

impl_address_format!(NetworkAddress);
impl_address_format!(BroadcastAddress);

impl TryFrom<ShortId> for NetworkAddress {
    type Error = ShortId;

    fn try_from(address: ShortId) -> Result<Self, Self::Error> {
        Self::new(address.as_u16()).ok_or(address)
    }
}

impl From<IndividualEndpoint> for Endpoint {
    fn from(endpoint: IndividualEndpoint) -> Self {
        endpoint.0
    }
}

impl TryFrom<Endpoint> for IndividualEndpoint {
    type Error = Endpoint;

    fn try_from(endpoint: Endpoint) -> Result<Self, Self::Error> {
        Self::new(endpoint).ok_or(endpoint)
    }
}

#[cfg(test)]
mod tests {
    use zb_core::Endpoint;

    use super::{
        AddressMode, BroadcastAddress, IndividualEndpoint, MAX_NETWORK_ADDRESS,
        MIN_BROADCAST_ADDRESS, NetworkAddress, RequestDestination,
    };

    #[test]
    fn network_address_rejects_reserved_and_broadcast_values() {
        assert!(NetworkAddress::new(MAX_NETWORK_ADDRESS).is_some());
        assert!(NetworkAddress::new(MAX_NETWORK_ADDRESS + 1).is_none());
    }

    #[test]
    fn broadcast_address_uses_the_apsde_multicast_range() {
        assert!(BroadcastAddress::new(MIN_BROADCAST_ADDRESS).is_some());
        assert!(BroadcastAddress::new(u16::MAX).is_some());
        assert!(BroadcastAddress::new(MIN_BROADCAST_ADDRESS - 1).is_none());
    }

    #[test]
    fn individual_endpoint_rejects_the_broadcast_endpoint() {
        assert!(IndividualEndpoint::new(Endpoint::Data).is_some());
        assert!(IndividualEndpoint::new(Endpoint::Broadcast).is_none());
    }

    #[test]
    fn request_destination_reports_its_address_mode() {
        assert_eq!(RequestDestination::Bound.mode(), AddressMode::Bound);
        assert_eq!(
            RequestDestination::Broadcast {
                address: zb_core::short_id::Broadcast::AllDevices,
                endpoint: Endpoint::Broadcast,
            }
            .mode(),
            AddressMode::Network
        );
    }
}
