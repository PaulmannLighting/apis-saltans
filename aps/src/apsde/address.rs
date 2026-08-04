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

/// "An endpoint that addresses one local application or the ZDO data service".
///
/// The APS broadcast endpoint is deliberately excluded.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IndividualEndpoint(Endpoint);

/// An individually addressed 16-bit NWK destination.
///
/// This type is useful for APS operations that require a response-capable
/// unicast destination. Unlike [`RequestDestination::Network`], it excludes
/// the APS broadcast endpoint.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NetworkDestination {
    address: NetworkAddress,
    endpoint: IndividualEndpoint,
}

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

    /// The received ASDU was addressed to a 16-bit NWK broadcast address and endpoint.
    Broadcast {
        /// NWK broadcast receiver set.
        address: short_id::Broadcast,
        /// Local target endpoint, including the APS broadcast endpoint.
        endpoint: Endpoint,
    },

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

impl NetworkDestination {
    /// Create an individually addressed NWK destination.
    #[must_use]
    pub const fn new(address: NetworkAddress, endpoint: IndividualEndpoint) -> Self {
        Self { address, endpoint }
    }

    /// Return the destination NWK address.
    #[must_use]
    pub const fn address(self) -> NetworkAddress {
        self.address
    }

    /// Return the destination endpoint.
    #[must_use]
    pub const fn endpoint(self) -> IndividualEndpoint {
        self.endpoint
    }
}

impl Display for NetworkDestination {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.address, formatter)?;
        formatter.write_str(":")?;
        Display::fmt(&self.endpoint.get(), formatter)
    }
}

impl LowerHex for NetworkDestination {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        LowerHex::fmt(&self.address, formatter)?;
        formatter.write_str(":")?;
        LowerHex::fmt(&self.endpoint.get(), formatter)
    }
}

impl UpperHex for NetworkDestination {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        UpperHex::fmt(&self.address, formatter)?;
        formatter.write_str(":")?;
        UpperHex::fmt(&self.endpoint.get(), formatter)
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

impl From<NetworkDestination> for RequestDestination {
    fn from(destination: NetworkDestination) -> Self {
        Self::Network {
            address: destination.address,
            endpoint: destination.endpoint.get(),
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
            Self::Broadcast { .. } | Self::Network { .. } => AddressMode::Network,
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

    /// Return the source NWK address when network addressing was used.
    #[must_use]
    pub const fn network_address(self) -> Option<NetworkAddress> {
        match self {
            Self::Network { address, .. } => Some(address),
            Self::Extended { .. } | Self::ExtendedWithoutEndpoint(_) => None,
        }
    }

    /// Return the source IEEE address when extended addressing was used.
    #[must_use]
    pub const fn ieee_address(self) -> Option<IeeeAddress> {
        match self {
            Self::Extended { address, .. } | Self::ExtendedWithoutEndpoint(address) => {
                Some(address)
            }
            Self::Network { .. } => None,
        }
    }

    /// Return the source endpoint when the source addressing mode includes one.
    #[must_use]
    pub const fn endpoint(self) -> Option<IndividualEndpoint> {
        match self {
            Self::Network { endpoint, .. } | Self::Extended { endpoint, .. } => Some(endpoint),
            Self::ExtendedWithoutEndpoint(_) => None,
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
    use zb_core::short_id::Broadcast;
    use zb_core::{Endpoint, IeeeAddress};

    use super::{
        AddressMode, BroadcastAddress, IndividualEndpoint, MAX_NETWORK_ADDRESS,
        MIN_BROADCAST_ADDRESS, NetworkAddress, ReceivedDestination, RequestDestination, Source,
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
    fn network_destination_converts_to_an_individual_request_destination() {
        const NETWORK_ADDRESS: u16 = 0x1234;

        let address = NetworkAddress::new(NETWORK_ADDRESS).expect("test NWK address is valid");
        let endpoint =
            IndividualEndpoint::new(Endpoint::Data).expect("data endpoint is individual");
        let destination = super::NetworkDestination::new(address, endpoint);

        assert_eq!(destination.address(), address);
        assert_eq!(destination.endpoint(), endpoint);
        assert_eq!(
            RequestDestination::from(destination),
            RequestDestination::Network {
                address,
                endpoint: Endpoint::Data,
            }
        );
    }

    #[test]
    fn received_broadcast_preserves_its_receiver_set_and_endpoint() {
        let destination = ReceivedDestination::Broadcast {
            address: Broadcast::RxOnWhenIdle,
            endpoint: Endpoint::Data,
        };

        assert_eq!(destination.mode(), AddressMode::Network);
        assert!(matches!(
            destination,
            ReceivedDestination::Broadcast {
                address: Broadcast::RxOnWhenIdle,
                endpoint: Endpoint::Data
            }
        ));
    }

    #[test]
    fn source_accessors_preserve_addressing_mode_fields() {
        const NETWORK_ADDRESS: u16 = 0x1234;
        const IEEE_ADDRESS: IeeeAddress = IeeeAddress::new(1, 2, 3, 4, 5, 6, 7, 8);

        let endpoint =
            IndividualEndpoint::new(Endpoint::Data).expect("data endpoint is individual");
        let network_address =
            NetworkAddress::new(NETWORK_ADDRESS).expect("test NWK address is valid");
        let network = Source::Network {
            address: network_address,
            endpoint,
        };
        let extended = Source::Extended {
            address: IEEE_ADDRESS,
            endpoint,
        };

        assert_eq!(network.network_address(), Some(network_address));
        assert_eq!(network.ieee_address(), None);
        assert_eq!(network.endpoint(), Some(endpoint));
        assert_eq!(extended.network_address(), None);
        assert_eq!(extended.ieee_address(), Some(IEEE_ADDRESS));
        assert_eq!(extended.endpoint(), Some(endpoint));
    }

    #[test]
    fn request_destination_reports_its_address_mode() {
        assert_eq!(RequestDestination::Bound.mode(), AddressMode::Bound);
        assert_eq!(
            RequestDestination::Broadcast {
                address: Broadcast::AllDevices,
                endpoint: Endpoint::Broadcast,
            }
            .mode(),
            AddressMode::Network
        );
    }
}
