//! Default Response Command.

use apis_saltans_core::Direction;

use crate::macros::zcl_command;

zcl_command! {
    /// Default Response Command
    DefaultResponse {
        Global;
        command_id: 0x0b;
        direction: Direction::ClientToServer;
        parse_direction: crate::ParseDirection::Both;
        disable_default_response: true;
        fields {
            command_id: u8,
            status: u8,
        }

        getters {
            /// Return the status of the default response.
            #[must_use]
            pub const fn status(&self) -> u8 {
                self.status
            }

            /// Return the command ID of the default response.
            #[must_use]
            pub const fn command_id(&self) -> u8 {
                self.command_id
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DefaultResponse;
    use crate::clusters::global;
    use crate::{Cluster, Command, Direction, Frame};

    #[test]
    fn uses_client_to_server_as_outgoing_direction() {
        assert_eq!(DefaultResponse::DIRECTION, Direction::ClientToServer);
    }

    #[test]
    fn parses_server_to_client_default_response() {
        let frame = Frame::parse(0x0008, [0x18, 0x06, 0x0b, 0x00, 0x00].into_iter())
            .expect("default response frame should parse");

        let Cluster::Global(global::Command::DefaultResponse(response)) = frame.into_payload()
        else {
            panic!("expected global default response");
        };

        assert_eq!(response.command_id(), 0x00);
        assert_eq!(response.status(), 0x00);
    }

    #[test]
    fn parses_client_to_server_default_response() {
        let frame = Frame::parse(0x0008, [0x10, 0x06, 0x0b, 0x01, 0x86].into_iter())
            .expect("default response frame should parse");

        let Cluster::Global(global::Command::DefaultResponse(response)) = frame.into_payload()
        else {
            panic!("expected global default response");
        };

        assert_eq!(response.command_id(), 0x01);
        assert_eq!(response.status(), 0x86);
    }
}
