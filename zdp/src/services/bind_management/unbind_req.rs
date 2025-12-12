use le_stream::{FromLeStream, ToLeStream};
use macaddr::MacAddr8;
use num_traits::FromPrimitive;
use zigbee::Cluster;

use super::{Address, AddressMode, Destination};
use crate::Service;

/// Request type for Bind Request.
#[derive(Clone, Debug, Eq, Hash, PartialEq, ToLeStream)]
pub struct UnbindReq {
    src_address: MacAddr8,
    src_endpoint: u8,
    cluster_id: u16,
    dst_addr_mode: u8,
    dst_address: Address,
    dst_endpoint: Option<u8>,
}

impl UnbindReq {
    /// Creates a new `UnbindReq`.
    #[must_use]
    pub const fn new(
        src_address: MacAddr8,
        src_endpoint: u8,
        cluster_id: u16,
        destination: Destination,
    ) -> Self {
        let (dst_address, dst_endpoint) = match destination {
            Destination::Group(group_addr) => (Address::Group(group_addr), None),
            Destination::Extended { address, endpoint } => {
                (Address::Extended(address), Some(endpoint))
            }
        };

        Self {
            src_address,
            src_endpoint,
            cluster_id,
            dst_addr_mode: destination.discriminant(),
            dst_address,
            dst_endpoint,
        }
    }

    /// Returns the source address.
    #[must_use]
    pub const fn src_address(&self) -> MacAddr8 {
        self.src_address
    }

    /// Returns the source endpoint.
    #[must_use]
    pub const fn src_endpoint(&self) -> u8 {
        self.src_endpoint
    }

    /// Returns the cluster ID.
    #[must_use]
    pub const fn cluster_id(&self) -> u16 {
        self.cluster_id
    }

    /// Returns the destination.
    #[expect(clippy::missing_panics_doc)]
    #[must_use]
    pub const fn destination(&self) -> Destination {
        match &self.dst_address {
            Address::Group(addr) => Destination::Group(*addr),
            Address::Extended(addr) => Destination::Extended {
                address: *addr,
                endpoint: self
                    .dst_endpoint
                    .expect("Extended address is guaranteed to have an endpoint"),
            },
        }
    }
}

impl Cluster for UnbindReq {
    const ID: u16 = 0x0022;
}

impl Service for UnbindReq {
    const NAME: &'static str = "Unbind_req";
}

impl FromLeStream for UnbindReq {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let src_address = MacAddr8::from_le_stream(&mut bytes)?;
        let src_endpoint = u8::from_le_stream(&mut bytes)?;
        let cluster_id = u16::from_le_stream(&mut bytes)?;
        let dst_addr_mode = u8::from_le_stream(&mut bytes)?;
        let (dst_address, dst_endpoint) = match AddressMode::from_u8(dst_addr_mode)? {
            AddressMode::Group => (u16::from_le_stream(&mut bytes).map(Address::Group)?, None),
            AddressMode::Extended => (
                MacAddr8::from_le_stream(&mut bytes).map(Address::Extended)?,
                Some(u8::from_le_stream(&mut bytes)?),
            ),
        };
        Some(Self {
            src_address,
            src_endpoint,
            cluster_id,
            dst_addr_mode,
            dst_address,
            dst_endpoint,
        })
    }
}
