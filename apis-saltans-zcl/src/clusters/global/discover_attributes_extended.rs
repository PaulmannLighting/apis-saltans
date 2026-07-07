//! Discover Attributes Extended Command and Response.

use std::boxed::Box;

use apis_saltans_core::Direction;

pub use self::attribute_information::AttributeInformation;
use crate::macros::zcl_command;

mod attribute_information;

zcl_command! {
    /// Discover Attributes Extended Command.
    Command {
        Global;
        command_id: 0x15;
        direction: Direction::ClientToServer;
        response: Response;
        => crate::global::DiscoverAttributesExtended;
        fields {
            start_attribute_id: u16,
            maximum_attribute_ids: u8,
        }
    }
}

zcl_command! {
    /// Discover Attributes Extended Response Command.
    Response {
        Global;
        command_id: 0x16;
        direction: Direction::ServerToClient;
        => crate::global::DiscoverAttributesExtendedResponse;
        fields {
            discovery_complete: u8,
            attributes: Box<[AttributeInformation]>,
        }

        getters {
            /// Return whether extended attribute discovery is complete.
            #[must_use]
            pub const fn discovery_complete(&self) -> u8 {
                self.discovery_complete
            }

            /// Return the discovered extended attribute information records.
            #[must_use]
            pub fn attributes(&self) -> &[AttributeInformation] {
                &self.attributes
            }
        }
    }
}
