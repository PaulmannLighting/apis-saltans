use std::iter::Chain;

use apis_saltans_core::node::Descriptor;
use apis_saltans_core::types::tlv::Tlv;
use le_stream::ToLeStream;

use crate::{Command, DeviceAndServiceDiscovery, Status};

crate::services::zdp_command! {
    /// Node Descriptor Response structure.
    NodeDescRsp => Node_Desc_rsp;
    cluster_id: 0x8002;
    fields {
        nwk_addr_of_interest: u16,
        node_descriptor: Result<Descriptor, Result<Status, u8>>,
        tlvs: Vec<Tlv>,
    }
    constructor {
        /// Creates a new `NodeDescRsp`.
        #[must_use]
        pub const fn new(
            nwk_addr_of_interest: u16,
            node_descriptor: Result<Descriptor, Status>,
            tlvs: Vec<Tlv>,
        ) -> Self {
            Self {
                nwk_addr_of_interest,
                node_descriptor: match node_descriptor {
                    Ok(node_descriptor) => Ok(node_descriptor),
                    Err(status) => Err(Ok(status)),
                },
                tlvs,
            }
        }
    }
    getters {
        /// Returns the network address of interest.
        #[must_use]
        pub const fn nwk_addr_of_interest(&self) -> u16 {
            self.nwk_addr_of_interest
        }

        /// Returns the descriptor result and TLVs.
        pub fn into_parts(self) -> (Result<Descriptor, Result<Status, u8>>, Vec<Tlv>) {
            (self.node_descriptor, self.tlvs)
        }
    }
    display {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{} {{ nwk_addr_of_interest: {:#06X}, ",
                Self::NAME,
                self.nwk_addr_of_interest,
            )?;

            match &self.node_descriptor {
                Ok(node_descriptor) => write!(
                    f,
                    "status: {}, node_descriptor: {:?}, ",
                    Status::Success,
                    node_descriptor
                )?,
                Err(Ok(status)) => write!(f, "status: {status:?}, ")?,
                Err(Err(status)) => write!(f, "status: {status:#04X}, ")?,
            }

            write!(f, "tlvs: [")?;

            let mut tlvs = self.tlvs.iter();

            if let Some(tlv) = tlvs.next() {
                write!(f, "{tlv:?}")?;

                for tlv in tlvs {
                    write!(f, ", {tlv:?}")?;
                }
            }

            write!(f, "] }}")
        }
    }
    le_stream {
        from {
            /// Parse a `NodeDescRsp` from the given byte stream.
            ///
            /// # Examples
            ///
            /// ```
            /// use le_stream::FromLeStream;
            ///
            /// use apis_saltans_zdp::{Frame, NodeDescRsp};
            ///
            /// let bytes: [u8; _] = [5, 0, 62, 199, 1, 64, 142, 24, 18, 66, 66, 0, 0, 42, 66, 0, 0];
            /// let node_desc_rsp = Frame::<NodeDescRsp>::from_le_stream(bytes.into_iter()).unwrap();
            /// ```
            fn from_le_stream<T>(mut bytes: T) -> Option<Self>
            where
                T: Iterator<Item = u8>,
            {
                let status = Status::try_from(u8::from_le_stream(&mut bytes)?);
                let nwk_addr_of_interest = u16::from_le_stream(&mut bytes)?;

                let node_descriptor = if status == Ok(Status::Success) {
                    Ok(Descriptor::from_le_stream(&mut bytes)?)
                } else {
                    Err(status)
                };

                let tlvs = Vec::from_le_stream(&mut bytes)?;

                Some(Self {
                    nwk_addr_of_interest,
                    node_descriptor,
                    tlvs,
                })
            }
        }
        to {
            type Iter = Chain<
                Chain<
                    Chain<<u8 as ToLeStream>::Iter, <u16 as ToLeStream>::Iter>,
                    <Option<Descriptor> as ToLeStream>::Iter,
                >,
                <Vec<Tlv> as ToLeStream>::Iter,
            >;

            fn to_le_stream(self) -> Self::Iter {
                let (status, descriptor) = match self.node_descriptor {
                    Ok(node_descriptor) => (u8::from(Status::Success), Some(node_descriptor)),
                    Err(Ok(status)) => (u8::from(status), None),
                    Err(Err(status)) => (status, None),
                };

                status
                    .to_le_stream()
                    .chain(self.nwk_addr_of_interest.to_le_stream())
                    .chain(descriptor.to_le_stream())
                    .chain(self.tlvs.to_le_stream())
            }
        }
    }
}

impl TryFrom<Command> for NodeDescRsp {
    type Error = Command;

    fn try_from(cmd: Command) -> Result<Self, Self::Error> {
        match cmd {
            Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::NodeDescRsp(rsp)) => {
                Ok(rsp)
            }
            other => Err(other),
        }
    }
}

impl TryFrom<NodeDescRsp> for Descriptor {
    type Error = Result<Status, u8>;

    fn try_from(value: NodeDescRsp) -> Result<Self, Self::Error> {
        value.node_descriptor
    }
}
