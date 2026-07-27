use zb_core::IeeeAddress;

use crate::Channel;

/// Information about a found network during a network scan.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NetworkDescriptor {
    channel: Channel,
    pan_id: u16,
    extended_pan_id: IeeeAddress,
    permits_joining: bool,
    stack_profile: u8,
    nwk_update_id: u8,
}

impl NetworkDescriptor {
    /// Create a discovered network descriptor.
    #[must_use]
    pub const fn new(
        channel: Channel,
        pan_id: u16,
        extended_pan_id: IeeeAddress,
        permits_joining: bool,
        stack_profile: u8,
        nwk_update_id: u8,
    ) -> Self {
        Self {
            channel,
            pan_id,
            extended_pan_id,
            permits_joining,
            stack_profile,
            nwk_update_id,
        }
    }

    /// Get the channel of the found network.
    #[must_use]
    pub const fn channel(&self) -> Channel {
        self.channel
    }

    /// Get the PAN ID of the found network.
    #[must_use]
    pub const fn pan_id(&self) -> u16 {
        self.pan_id
    }

    /// Return the network's extended PAN ID.
    #[must_use]
    pub const fn extended_pan_id(&self) -> IeeeAddress {
        self.extended_pan_id
    }

    /// Check if the found network allows joins.
    #[must_use]
    pub const fn permits_joining(&self) -> bool {
        self.permits_joining
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
