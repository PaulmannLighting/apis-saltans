use apis_saltans_core::ByteSizedVec;
use le_stream::{Consume, ToLeStream};

use crate::{SimpleDescriptor, Status};

crate::zdp_command! {
    /// Simple Descriptor Response.
    SimpleDescRsp => Simple_Desc_rsp;
    cluster_id: 0x8004;
    group: DeviceAndServiceDiscovery;
    fields {
        status: u8,
        nwk_addr_of_interest: u16,
        descriptor: ByteSizedVec<u8>,
    }
    constructor {
        /// Creates a new Simple Descriptor Response.
        #[must_use]
        pub fn new(
            nwk_addr_of_interest: u16,
            descriptor: Result<SimpleDescriptor, Status>,
        ) -> Self {
            match descriptor {
                Ok(descriptor) => Self {
                    status: Status::Success.into(),
                    nwk_addr_of_interest,
                    descriptor: descriptor.to_le_stream().collect(),
                },
                Err(status) => Self {
                    status: status.into(),
                    nwk_addr_of_interest,
                    descriptor: ByteSizedVec::new(),
                },
            }
        }
    }
    getters {
        /// Return the status of the response.
        ///
        /// # Errors
        ///
        /// Returns the raw status code if the conversion to a [`Status`] fails.
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
        pub fn descriptor(&self) -> &[u8] {
            &self.descriptor
        }

        /// Return the descriptor object, consuming the `SimpleDescRsp`.
        ///
        /// Returns `None` if the descriptor bytes cannot be parsed into a `SimpleDescriptor`.
        pub fn into_descriptor(self) -> Option<SimpleDescriptor> {
            self.descriptor.into_iter().consume().map_err(drop).ok()
        }
    }

    display {
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

}
