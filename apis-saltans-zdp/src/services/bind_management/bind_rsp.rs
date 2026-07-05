use crate::{Displayable, Status};

crate::zdp_command! {
    /// Binding response.
    derive { Copy }
    BindRsp => Bind_rsp;
    cluster_id: 0x8021;
    group: BindManagement;
    fields {
        status: u8,
    }
    constructor {
        /// Creates a new `BindRsp`.
        #[must_use]
        pub const fn new(status: Status) -> Self {
            Self {
                status: status as u8,
            }
        }
    }
    getters {
        /// Returns the status.
        ///
        /// # Errors
        ///
        /// Returns an error if the status code is invalid.
        pub fn status(self) -> Result<Status, u8> {
            self.status.try_into()
        }
    }
    display {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{} {{ status: {} }}",
                Self::NAME,
                self.status().display()
            )
        }
    }
}
