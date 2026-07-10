use crate::Status;

crate::zdp_command! {
    /// Power Descriptor Response.
    derive { Copy }
    PowerDescRsp => Power_Desc_rsp;
    cluster_id: 0x8003;
    group: DeviceAndServiceDiscovery;
    fields {
        status: u8,
        nwk_addr_of_interest: u16,
        power_descriptor: Option<u16>,
    }
    constructor {
        /// Creates a new `PowerDescRsp`.
        #[must_use]
        pub const fn new(nwk_addr_of_interest: u16, power_descriptor: Result<u16, Status>) -> Self {
            match power_descriptor {
                Ok(power_descriptor) => Self {
                    status: Status::Success as u8,
                    nwk_addr_of_interest,
                    power_descriptor: Some(power_descriptor),
                },
                Err(status) => Self {
                    status: status as u8,
                    nwk_addr_of_interest,
                    power_descriptor: None,
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
    }
}
