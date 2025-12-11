use std::iter::Chain;

use le_stream::{FromLeStream, ToLeStream};

pub use self::fragmentation_options::FragmentationOptions;
use crate::types::tlv::Tag;

mod fragmentation_options;

/// Fragmentation Parameters TLV structure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream)]
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

    fn size(&self) -> usize {
        let mut size = 2; // node_id size

        if self.options.is_some() {
            size += 1;
        }

        if self.maximum_incoming_transfer_unit.is_some() {
            size += 2;
        }

        size
    }
}

impl ToLeStream for FragmentationParameters {
    type Iter = Chain<
        Chain<
            Chain<
                Chain<<u8 as ToLeStream>::Iter, <u8 as ToLeStream>::Iter>,
                <u16 as ToLeStream>::Iter,
            >,
            <Option<FragmentationOptions> as ToLeStream>::Iter,
        >,
        <Option<u16> as ToLeStream>::Iter,
    >;

    fn to_le_stream(self) -> Self::Iter {
        Self::TAG
            .to_le_stream()
            .chain(self.serialized_size().to_le_stream())
            .chain(self.node_id.to_le_stream())
            .chain(self.options.to_le_stream())
            .chain(self.maximum_incoming_transfer_unit.to_le_stream())
    }
}
