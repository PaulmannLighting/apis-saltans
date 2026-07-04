use crate::{ByteSizedVec, Command, Displayable, Status};

crate::services::zdp_command! {
    /// Match Descriptor Response.
    MatchDescRsp => Match_Desc_rsp;
    cluster_id: 0x8006;
    fields {
        status: u8,
        nwk_addr_of_interest: u16,
        matches: ByteSizedVec<u8>,
    }
    constructor {
        /// Creates a new `MatchDescRsp`.
        #[must_use]
        pub const fn new(
            status: Status,
            nwk_addr_of_interest: u16,
            matches: ByteSizedVec<u8>,
        ) -> Self {
            Self {
                status: status as u8,
                nwk_addr_of_interest,
                matches,
            }
        }
    }
    getters {
        /// Returns the status.
        ///
        /// # Errors
        ///
        /// Returns an error if the status code is invalid.
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
    from {
        impl From<MatchDescRsp> for Command {
            fn from(value: MatchDescRsp) -> Self {
                Self::DeviceAndServiceDiscovery(value.into())
            }
        }
    }
}
