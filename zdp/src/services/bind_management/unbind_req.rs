use zb_core::endpoint::Reserved;
use zb_core::{Endpoint, IeeeAddress};

use super::{Address, AddressMode, Destination};

crate::zdp_command! {
    /// Request type for Bind Request.
    UnbindReq => Unbind_req;
    cluster_id: 0x0022;
    group: BindManagement;
    response: crate::UnbindRsp;
    fields {
        src_address: IeeeAddress,
        src_endpoint: u8,
        cluster_id: u16,
        dst_addr_mode: u8,
        dst_address: Address,
        dst_endpoint: Option<u8>,
    }
    constructor {
        /// Creates a new `UnbindReq`.
        #[must_use]
        pub const fn new(
            src_address: IeeeAddress,
            src_endpoint: Endpoint,
            cluster_id: u16,
            destination: Destination,
        ) -> Self {
            let (dst_address, dst_endpoint) = match destination {
                Destination::Group(group_addr) => (Address::Group(group_addr), None),
                Destination::Extended { address, endpoint } => {
                    (Address::Extended(address), Some(endpoint.as_u8()))
                }
            };

            Self {
                src_address,
                src_endpoint: src_endpoint.as_u8(),
                cluster_id,
                dst_addr_mode: destination.discriminant(),
                dst_address,
                dst_endpoint,
            }
        }
    }
    getters {
        /// Returns the source address.
        #[must_use]
        pub const fn src_address(&self) -> IeeeAddress {
            self.src_address
        }

        /// Returns the source endpoint.
        ///
        /// # Errors
        ///
        /// Returns [`Reserved`] if the raw endpoint value is reserved.
        pub fn src_endpoint(&self) -> Result<Endpoint, Reserved> {
            self.src_endpoint.try_into()
        }

        /// Returns the cluster ID.
        #[must_use]
        pub const fn cluster_id(&self) -> u16 {
            self.cluster_id
        }

        /// Returns the destination endpoint, if present.
        pub fn dst_endpoint(&self) -> Option<Result<Endpoint, Reserved>> {
            self.dst_endpoint.map(TryInto::try_into)
        }

        /// Returns the destination.
        #[expect(clippy::missing_panics_doc)]
        ///
        /// # Errors
        ///
        /// Returns [`Reserved`] if the raw destination endpoint value is reserved.
        pub fn destination(&self) -> Result<Destination, Reserved> {
            match &self.dst_address {
                Address::Group(addr) => Ok(Destination::Group(*addr)),
                Address::Extended(addr) => Ok(Destination::Extended {
                    address: *addr,
                    endpoint: self
                        .dst_endpoint
                        .expect("Extended address is guaranteed to have an endpoint")
                        .try_into()?,
                }),
            }
        }
    }
    display {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{} {{ src_address: {}, src_endpoint: {}, cluster_id: {:#06X}, destination: {} }}",
                Self::NAME,
                self.src_address,
                self.src_endpoint,
                self.cluster_id,
                self.destination().map_err(|_| std::fmt::Error)?
            )
        }
    }
    le_stream {
        from {
            fn from_le_stream<T>(mut bytes: T) -> Option<Self>
            where
                T: Iterator<Item = u8>,
            {
                let src_address = IeeeAddress::from_le_stream(&mut bytes)?;
                let src_endpoint = u8::from_le_stream(&mut bytes)?;
                let cluster_id = u16::from_le_stream(&mut bytes)?;
                let dst_addr_mode = u8::from_le_stream(&mut bytes)?;
                let (dst_address, dst_endpoint) = match AddressMode::try_from(dst_addr_mode).ok()? {
                    AddressMode::Group => {
                        (u16::from_le_stream(&mut bytes).map(Address::Group)?, None)
                    }
                    AddressMode::Extended => (
                        IeeeAddress::from_le_stream(&mut bytes).map(Address::Extended)?,
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
    }
}
