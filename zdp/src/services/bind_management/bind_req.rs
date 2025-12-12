use le_stream::{FromLeStream, ToLeStream};
use log::warn;
use macaddr::MacAddr8;
use zigbee::Cluster;

use self::address::Address;
pub use self::destination::Destination;
use crate::Service;

mod address;
mod destination;

const GROUP_ADDRESS_MODE: u8 = 0x01;
const EXTENDED_ADDRESS_MODE: u8 = 0x03;

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
            GROUP_ADDRESS_MODE => u16::from_le_stream(&mut bytes).map(Address::Group)?,
            EXTENDED_ADDRESS_MODE => MacAddr8::from_le_stream(&mut bytes).map(Address::Extended)?,
            _ => {
                warn!(
                    "Received {} with invalid destination address mode: {dst_addr_mode}",
                    Self::NAME
                );
                return None;
            }
        };
        let dst_endpoint = if dst_addr_mode == EXTENDED_ADDRESS_MODE {
            Some(u8::from_le_stream(&mut bytes)?)
        } else {
            None
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
