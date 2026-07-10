use le_stream::{FromLeStream, ToLeStream};

pub use self::fragmentation_options::FragmentationOptions;
use crate::types::tlv::Tag;

mod fragmentation_options;

/// Fragmentation Parameters TLV structure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct FragmentationParameters {
    node_id: u16,
    options: Option<FragmentationOptions>,
    maximum_incoming_transfer_unit: Option<u16>,
}

impl FragmentationParameters {
    /// Create a new `FragmentationParameters`.
    #[must_use]
    pub const fn new(
        node_id: u16,
        options: Option<FragmentationOptions>,
        maximum_incoming_transfer_unit: Option<u16>,
    ) -> Self {
        Self {
            node_id,
            options,
            maximum_incoming_transfer_unit,
        }
    }

    /// Get the Node ID.
    #[must_use]
    pub const fn node_id(self) -> u16 {
        self.node_id
    }

    /// Get the Options.
    #[must_use]
    pub const fn options(self) -> Option<FragmentationOptions> {
        self.options
    }

    /// Get the Maximum Incoming Transfer Unit.
    #[must_use]
    pub const fn maximum_incoming_transfer_unit(self) -> Option<u16> {
        self.maximum_incoming_transfer_unit
    }
}

impl Tag for FragmentationParameters {
    const TAG: u8 = 71;
}
