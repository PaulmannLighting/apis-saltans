use apis_saltans_core::ByteSizedVec;

use crate::{Displayable, Status};

crate::zdp_command! {
    /// Match Descriptor Response.
    MatchDescRsp => Match_Desc_rsp;
    cluster_id: 0x8006;
    group: DeviceAndServiceDiscovery;
    fields {
        status: u8,
        nwk_addr_of_interest: u16,
        matches: ByteSizedVec<u8>,
    }
    constructor {
        /// Creates a new `MatchDescRsp`.
        #[must_use]
        pub fn new(
            nwk_addr_of_interest: u16,
            matches: Result<ByteSizedVec<u8>, Status>,
        ) -> Self {
            match matches {
                Ok(matches) => Self {
                    status: Status::Success as u8,
                    nwk_addr_of_interest,
                    matches,
                },
                Err(status) => Self {
                    status: status as u8,
                    nwk_addr_of_interest,
                    matches: ByteSizedVec::new(),
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

        /// Returns the network address of interest.
        #[must_use]
        pub const fn nwk_addr_of_interest(&self) -> u16 {
            self.nwk_addr_of_interest
        }

        /// Returns a reference to the list of matched endpoints.
        #[must_use]
        pub fn matches(&self) -> &[u8] {
            &self.matches
        }
    }
    display {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{} {{ status: {}, nwk_addr_of_interest: {:#06X}, matches: [",
                Self::NAME,
                self.status().display(),
                self.nwk_addr_of_interest,
            )?;

            let mut endpoints = self.matches.iter();

            if let Some(endpoint) = endpoints.next() {
                write!(f, "{endpoint:#04X}")?;

                for endpoint in endpoints {
                    write!(f, ", {endpoint:#04X}")?;
                }
            }

            write!(f, "] }}")
        }
    }
}
