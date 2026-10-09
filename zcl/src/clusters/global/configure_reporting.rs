//! Reporting configuration commands for the Global cluster.

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
    /// Configure Reporting response containing success or failed attribute configurations.
    ///
    /// Records are preserved without validating response-level protocol constraints.
    Response {
        Global;
        command_id: 0x07;
        direction: Direction::ServerToClient;
        => crate::global::ConfigureReportingResponse;
        fields {
            status: Box<[AttributeStatus]>,
        }

        getters {
            /// Returns attribute status records.
            ///
            /// Records retain their original order, including mixed successes and failures.
            #[must_use]
            pub fn status(&self) -> &[AttributeStatus] {
                &self.status
            }

            /// Returns whether all records indicate success, including an empty list.
            #[must_use]
            pub fn is_success(&self) -> bool {
                self.status.iter().all(|record| record.status() == crate::Status::Success as u8)
            }
        }

    }
}

#[cfg(test)]
mod tests {
    use le_stream::{FromLeStream, ToLeStream};
    use zb_core::Direction;
    use zb_core::types::{Bool, Uint16};

    use super::{AttributeStatus, Receive, Response, Send, receive};
    use crate::clusters::general::{level, on_off};
    use crate::{Analog, Directed, Discrete, Status};

    const SEND_ATTRIBUTE_ID: u16 = 0x0000;
    const ANALOG_SEND_ATTRIBUTE_ID: u16 = 0x0004;
    const SEND_ATTRIBUTE_DATA_TYPE: u8 = 0x10;
    const RECEIVE_ATTRIBUTE_ID: u16 = 0x1234;
    const MINIMUM_REPORTING_INTERVAL: u16 = 0x0102;
    const MAXIMUM_REPORTING_INTERVAL: u16 = 0x0304;
    const TIMEOUT_PERIOD: u16 = 0x0506;

    #[test]
    fn parses_and_serializes_success_response() {
        let response = Response::from_le_stream([0x00].into_iter()).unwrap();
        assert!(response.is_success());
        assert_eq!(response.status(), [AttributeStatus::success()]);
        assert_eq!(response.to_le_stream().collect::<Vec<_>>(), [0x00]);
    }

    #[test]
    fn parses_success_response_through_frame_dispatch() {
        const CLUSTER_ID: u16 = 0x0006;
        let frame = crate::Frame::parse(CLUSTER_ID, [0x18, 0x42, 0x07, 0x00].into_iter()).unwrap();
        let response = Response::try_from(frame.into_payload()).unwrap();
        assert!(response.is_success());
        assert_eq!(response.status(), [AttributeStatus::success()]);
    }

    #[test]
    fn preserves_failure_response_records() {
        const PAYLOAD: [u8; 8] = [0x86, 0x00, 0x34, 0x12, 0x8c, 0x01, 0x78, 0x56];
        let response = Response::from_le_stream(PAYLOAD.into_iter()).unwrap();
        assert!(!response.is_success());
        assert_eq!(
            response.status(),
            [
                AttributeStatus::new(Status::UnsupportedAttribute, Some(0x00), Some(0x1234)),
                AttributeStatus::new(Status::UnreportableAttribute, Some(0x01), Some(0x5678)),
            ]
        );
        assert_eq!(response.to_le_stream().collect::<Vec<_>>(), PAYLOAD);
    }

    #[test]
    fn parses_mixed_records_and_serializes_success_without_fields() {
        const PAYLOAD: [u8; 9] = [0x00, 0x01, 0x34, 0x12, 0x86, 0x00, 0x78, 0x56, 0x00];
        let response = Response::from_le_stream(PAYLOAD.into_iter()).unwrap();
        assert_eq!(response.status().len(), 3);
        assert_eq!(response.status()[0].direction(), Some(0x01));
        assert_eq!(response.status()[0].attribute_id(), Some(0x1234));
        assert_eq!(
            response.status()[1],
            AttributeStatus::new(Status::UnsupportedAttribute, Some(0x00), Some(0x5678))
        );
        assert_eq!(response.status()[2], AttributeStatus::success());
        assert!(!response.is_success());
        assert_eq!(Response::new(response.status().into()), response);
        assert_eq!(
            response.to_le_stream().collect::<Vec<_>>(),
            [0x00, 0x86, 0x00, 0x78, 0x56, 0x00],
        );
    }

    #[test]
    fn accepts_empty_response_and_missing_optional_fields() {
        const PAYLOADS: &[&[u8]] = &[&[], &[0x86], &[0x86, 0x00], &[0x86, 0x00, 0x34, 0x12, 0x8c]];
        for payload in PAYLOADS {
            let response = Response::from_le_stream(payload.iter().copied()).unwrap();
            let records =
                Box::<[AttributeStatus]>::from_le_stream(payload.iter().copied()).unwrap();
            assert_eq!(response.status(), &*records);
        }
    }

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
        let attribute = on_off::SendReport::OnOff(Discrete::<Bool>::new(
            MINIMUM_REPORTING_INTERVAL,
            MAXIMUM_REPORTING_INTERVAL,
        ))
        .into();
        let command = Send::new(Box::new([attribute]));
        let bytes: Vec<_> = command.clone().to_le_stream().collect();
        let mut expected = vec![Direction::ClientToServer as u8];
        expected.extend(SEND_ATTRIBUTE_ID.to_le_bytes());
        expected.push(SEND_ATTRIBUTE_DATA_TYPE);
        expected.extend(MINIMUM_REPORTING_INTERVAL.to_le_bytes());
        expected.extend(MAXIMUM_REPORTING_INTERVAL.to_le_bytes());

        assert_eq!(bytes, expected);
        assert_eq!(Send::from_le_stream(bytes.into_iter()), Some(command));
    }

    #[test]
    fn serializes_analog_reportable_change() {
        const REPORTABLE_CHANGE: u16 = 0x0506;

        let attribute = level::SendReport::CurrentFrequency(Analog::new(
            MINIMUM_REPORTING_INTERVAL,
            MAXIMUM_REPORTING_INTERVAL,
            Uint16::new(REPORTABLE_CHANGE),
        ))
        .into();
        let command = Send::new(Box::new([attribute]));
        let bytes: Vec<_> = command.to_le_stream().collect();
        let mut expected = vec![Direction::ClientToServer as u8];
        expected.extend(ANALOG_SEND_ATTRIBUTE_ID.to_le_bytes());
        expected.push(<Uint16 as zb_core::TypeId>::ID);
        expected.extend(MINIMUM_REPORTING_INTERVAL.to_le_bytes());
        expected.extend(MAXIMUM_REPORTING_INTERVAL.to_le_bytes());
        expected.extend(REPORTABLE_CHANGE.to_le_bytes());

        assert_eq!(bytes, expected);
    }

    #[test]
    fn serializes_and_parses_receive_command() {
        let attribute =
            receive::AttributeReportingConfiguration::new(RECEIVE_ATTRIBUTE_ID, TIMEOUT_PERIOD);
        let command = Receive::new(Box::new([attribute]));
        let bytes: Vec<_> = command.clone().to_le_stream().collect();
        let mut expected = vec![Direction::ServerToClient as u8];
        expected.extend(RECEIVE_ATTRIBUTE_ID.to_le_bytes());
        expected.extend(TIMEOUT_PERIOD.to_le_bytes());

        assert_eq!(bytes, expected);
        assert_eq!(Receive::from_le_stream(bytes.into_iter()), Some(command));
    }
}
