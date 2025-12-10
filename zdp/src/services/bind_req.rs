use le_stream::{FromLeStream, ToLeStream};
use macaddr::MacAddr8;
use zigbee::Cluster;

use self::address::Address;
pub use self::destination::Destination;
use crate::Service;

mod address;
mod destination;

/// Request type for Bind Request.
#[derive(Clone, Debug, Eq, Hash, PartialEq, ToLeStream)]
pub struct BindReq {
    src_address: MacAddr8,
    src_endpoint: u8,
    cluster_id: u16,
    dst_addr_mode: u8,
    dst_address: Address,
    dst_endpoint: Option<u8>,
}

impl BindReq {
    /// Creates a new `BindReq`.
    #[must_use]
    pub const fn new(
        src_address: MacAddr8,
        src_endpoint: u8,
        cluster_id: u16,
        destination: Destination,
    ) -> Self {
        match destination {
            Destination::Group(group_addr) => Self {
                src_address,
                src_endpoint,
                cluster_id,
                dst_addr_mode: destination.discriminant(),
                dst_address: Address::Group(group_addr),
                dst_endpoint: None,
            },
            Destination::Extended { address, endpoint } => Self {
                src_address,
                src_endpoint,
                cluster_id,
                dst_addr_mode: destination.discriminant(),
                dst_address: Address::Extended(address),
                dst_endpoint: Some(endpoint),
            },
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
    ///
    /// # Returns
    ///
    /// `Some(Destination)` if the destination is valid, otherwise `None`.
    #[must_use]
    pub const fn destination(&self) -> Option<Destination> {
        match &self.dst_address {
            Address::Group(addr) => Some(Destination::Group(*addr)),
            Address::Extended(addr) => {
                if let Some(endpoint) = self.dst_endpoint {
                    Some(Destination::Extended {
                        address: *addr,
                        endpoint,
                    })
                } else {
                    None
                }
            }
        }
    }
}

impl Cluster for BindReq {
    const ID: u16 = 0x0021;
}

impl Service for BindReq {
    const NAME: &'static str = "Bind_req";
}

// TODO: Use constants for magic numbers.
impl FromLeStream for BindReq {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let src_address = MacAddr8::from_le_stream(&mut bytes)?;
        let src_endpoint = u8::from_le_stream(&mut bytes)?;
        let cluster_id = u16::from_le_stream(&mut bytes)?;
        let dst_addr_mode = u8::from_le_stream(&mut bytes)?;
        let dst_address = match dst_addr_mode {
            0x01 => {
                let group_addr = u16::from_le_stream(&mut bytes)?;
                Address::Group(group_addr)
            }
            0x03 => {
                let extended_addr = MacAddr8::from_le_stream(&mut bytes)?;
                Address::Extended(extended_addr)
            }
            _ => return None,
        };
        let dst_endpoint = match dst_addr_mode {
            0x03 => Some(u8::from_le_stream(&mut bytes)?),
            _ => None,
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
