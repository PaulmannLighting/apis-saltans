//! Reporting configuration commands for the Global cluster.

use std::boxed::Box;

use zb_core::Direction;

pub use self::attribute_status::AttributeStatus;
pub use self::receive::Command as Receive;
pub use self::send::Command as Send;
use crate::macros::zcl_command;

mod attribute_status;
pub mod receive;
pub mod send;

const COMMAND_ID: u8 = 0x06;

zcl_command! {
    /// Status of an attribute reporting configuration.
    Response {
        Global;
        command_id: 0x07;
        direction: Direction::ServerToClient;
        => crate::global::ConfigureReportingResponse;
        fields {
            status: Box<[AttributeStatus]>,
        }

        getters {
            /// Returns the status.
            #[must_use]
            pub fn status(&self) -> &[AttributeStatus] {
                &self.status
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use le_stream::{FromLeStream, ToLeStream};
    use zb_core::Direction;

    use super::{Receive, Send, receive, send};
    use crate::Directed;

    const ATTRIBUTE_ID: u16 = 0x1234;
    const ATTRIBUTE_DATA_TYPE: u8 = 0x20;
    const MINIMUM_REPORTING_INTERVAL: u16 = 0x0102;
    const MAXIMUM_REPORTING_INTERVAL: u16 = 0x0304;
    const TIMEOUT_PERIOD: u16 = 0x0506;

    #[test]
    fn send_command_has_client_to_server_direction() {
        assert_eq!(Send::DIRECTION, Direction::ClientToServer);
    }

    #[test]
    fn receive_command_has_server_to_client_direction() {
        assert_eq!(Receive::DIRECTION, Direction::ServerToClient);
    }

    #[test]
    fn serializes_and_parses_send_command() {
        let attribute = send::AttributeReportingConfiguration::new(
            ATTRIBUTE_ID,
            ATTRIBUTE_DATA_TYPE,
            MINIMUM_REPORTING_INTERVAL,
            MAXIMUM_REPORTING_INTERVAL,
            None,
        );
        let command = Send::new(Box::new([attribute]));
        let bytes: Vec<_> = command.clone().to_le_stream().collect();
        let mut expected = vec![Direction::ClientToServer as u8];
        expected.extend(ATTRIBUTE_ID.to_le_bytes());
        expected.push(ATTRIBUTE_DATA_TYPE);
        expected.extend(MINIMUM_REPORTING_INTERVAL.to_le_bytes());
        expected.extend(MAXIMUM_REPORTING_INTERVAL.to_le_bytes());

        assert_eq!(bytes, expected);
        assert_eq!(Send::from_le_stream(bytes.into_iter()), Some(command));
    }

    #[test]
    fn serializes_and_parses_receive_command() {
        let attribute = receive::AttributeReportingConfiguration::new(ATTRIBUTE_ID, TIMEOUT_PERIOD);
        let command = Receive::new(Box::new([attribute]));
        let bytes: Vec<_> = command.clone().to_le_stream().collect();
        let mut expected = vec![Direction::ServerToClient as u8];
        expected.extend(ATTRIBUTE_ID.to_le_bytes());
        expected.extend(TIMEOUT_PERIOD.to_le_bytes());

        assert_eq!(bytes, expected);
        assert_eq!(Receive::from_le_stream(bytes.into_iter()), Some(command));
    }
}
