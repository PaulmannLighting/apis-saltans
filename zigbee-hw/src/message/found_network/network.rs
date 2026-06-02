use macaddr::MacAddr8;

/// Information about a found network during a network scan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Network {
    channel: u8,
    pan_id: u16,
    ieee_address: MacAddr8,
    allow_joins: bool,
    stack_profile: u8,
    nwk_update_id: u8,
}

impl Network {
    /// Create a new `FoundNetwork`.
    #[must_use]
    pub const fn new(
        channel: u8,
        pan_id: u16,
        ieee_address: MacAddr8,
        allow_joins: bool,
        stack_profile: u8,
        nwk_update_id: u8,
    ) -> Self {
        Self {
            channel,
            pan_id,
            ieee_address,
            allow_joins,
            stack_profile,
            nwk_update_id,
        }
    }

    /// Get the channel of the found network.
    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }

    /// Get the PAN ID of the found network.
    #[must_use]
    pub const fn pan_id(&self) -> u16 {
        self.pan_id
    }

    /// Get the IEEE address of the found network.
    #[must_use]
    pub const fn ieee_address(&self) -> MacAddr8 {
        self.ieee_address
    }

    /// Check if the found network allows joins.
    #[must_use]
    pub const fn allow_joins(&self) -> bool {
        self.allow_joins
    }

    /// Get the stack profile of the found network.
    #[must_use]
    pub const fn stack_profile(&self) -> u8 {
        self.stack_profile
    }

    /// Get the NWK update ID of the found network.
    #[must_use]
    pub const fn nwk_update_id(&self) -> u8 {
        self.nwk_update_id
    }
}
