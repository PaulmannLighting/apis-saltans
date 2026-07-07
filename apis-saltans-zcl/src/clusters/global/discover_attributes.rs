//! Discover Attributes Command and Response.

use std::boxed::Box;

use apis_saltans_core::Direction;

pub use self::attribute_information::AttributeInformation;
use crate::macros::zcl_command;

mod attribute_information;

zcl_command! {
    /// Discover Attributes Command.
    Command {
        Global;
        command_id: 0x0c;
        direction: Direction::ClientToServer;
        response: Response;
        => crate::global::DiscoverAttributes;
        fields {
            start_attribute_id: u16,
            maximum_attribute_ids: u8,
        }
    }
}

zcl_command! {
    /// Discover Attributes Response Command.
    Response {
        Global;
        command_id: 0x0d;
        direction: Direction::ServerToClient;
        => crate::global::DiscoverAttributesResponse;
        fields {
            discovery_complete: u8,
            attributes: Box<[AttributeInformation]>,
        }

        getters {
            /// Return whether attribute discovery is complete.
            #[must_use]
            pub const fn discovery_complete(&self) -> u8 {
                self.discovery_complete
            }

            /// Return the discovered attributes.
            #[must_use]
            pub fn attributes(&self) -> &[AttributeInformation] {
                &self.attributes
            }
        }
    }
}
