use le_stream::FromLeStream;

pub use self::fragmentation_options::FragmentationOptions;
use crate::types::tlv::Tag;

mod fragmentation_options;

/// Fragmentation Parameters TLV structure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream)]
pub struct FragmentationParameters {
    node_id: u16,
    options: FragmentationOptions,
    maximum_incoming_transfer_unit: u16,
}

impl FragmentationParameters {
    /// Get the Node ID.
    #[must_use]
    pub const fn node_id(self) -> u16 {
        self.node_id
    }

    /// Get the Options.
    #[must_use]
    pub const fn options(self) -> FragmentationOptions {
        self.options
    }

    /// Get the Maximum Incoming Transfer Unit.
    #[must_use]
    pub const fn maximum_incoming_transfer_unit(self) -> u16 {
        self.maximum_incoming_transfer_unit
    }
}

impl Tag for FragmentationParameters {
    const TAG: u8 = 71;
}
