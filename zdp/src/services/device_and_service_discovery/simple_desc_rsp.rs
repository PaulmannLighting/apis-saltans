use std::fmt::Display;
use std::iter::Chain;

use le_stream::{FromLeStream, Prefixed, ToLeStream};
use zigbee::Cluster;

use crate::{Command, DeviceAndServiceDiscovery, Service, SimpleDescriptor, Status};

/// Simple Descriptor Response.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SimpleDescRsp {
    status: u8,
    nwk_addr_of_interest: u16,
    descriptor: SimpleDescriptor,
}

impl SimpleDescRsp {
    /// Creates a new Simple Descriptor Response.
    #[must_use]
    pub fn new(status: Status, nwk_addr_of_interest: u16, descriptor: SimpleDescriptor) -> Self {
        Self {
            status: status.into(),
            nwk_addr_of_interest,
            descriptor,
        }
    }

    /// Return the status.
    ///
    /// # Errors
    ///
    /// Returns an error if the status code is not a valid `Status`.
    pub fn status(&self) -> Result<Status, u8> {
        self.status.try_into()
    }

    /// Return the network address of interest.
    #[must_use]
    pub const fn nwk_addr_of_interest(&self) -> u16 {
        self.nwk_addr_of_interest
    }

    /// Return the descriptors.
    #[must_use]
    pub const fn descriptor(&self) -> &SimpleDescriptor {
        &self.descriptor
    }

    /// Return the descriptors, consuming the descriptor.
    #[must_use]
    pub fn into_descriptor(self) -> SimpleDescriptor {
        self.descriptor
    }
}

impl Cluster for SimpleDescRsp {
    const ID: u16 = 0x8004;
}

impl Service for SimpleDescRsp {
    const NAME: &'static str = "Simple_Desc_rsp";
}

impl Display for SimpleDescRsp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {{ status: {:#04X}, nwk_addr_of_interest: {:#06X}, descriptor: {:?} }}",
            Self::NAME,
            self.status,
            self.nwk_addr_of_interest,
            self.descriptor
        )
    }
}

impl TryFrom<Command> for SimpleDescRsp {
    type Error = Command;

    fn try_from(cmd: Command) -> Result<Self, Self::Error> {
        if let Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::SimpleDescRsp(
            descriptors,
        )) = cmd
        {
            Ok(descriptors)
        } else {
            Err(cmd)
        }
    }
}

impl FromLeStream for SimpleDescRsp {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let status = u8::from_le_stream(&mut bytes)?;
        let nwk_addr_of_interest = u16::from_le_stream(&mut bytes)?;
        let descriptor_bytes = Prefixed::<u8, Box<[u8]>>::from_le_stream(&mut bytes)?;
        let descriptor = SimpleDescriptor::from_le_stream(descriptor_bytes.into_iter())?;
        Some(Self {
            status,
            nwk_addr_of_interest,
            descriptor,
        })
    }
}

impl ToLeStream for SimpleDescRsp {
    type Iter = Chain<
        Chain<Chain<<u8 as ToLeStream>::Iter, <u16 as ToLeStream>::Iter>, <u8 as ToLeStream>::Iter>,
        <Box<[u8]> as ToLeStream>::Iter,
    >;

    fn to_le_stream(self) -> Self::Iter {
        let descriptor_bytes: Box<[u8]> = self.descriptor.to_le_stream().collect();
        #[expect(clippy::cast_possible_truncation)]
        self.status
            .to_le_stream()
            .chain(self.nwk_addr_of_interest.to_le_stream())
            .chain((descriptor_bytes.len() as u8).to_le_stream())
            .chain(descriptor_bytes.to_le_stream())
    }
}
