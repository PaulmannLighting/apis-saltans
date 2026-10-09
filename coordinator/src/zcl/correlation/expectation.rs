use zb_zcl::global::default_response::DefaultResponse;
use zb_zcl::{Cluster, Frame, Header, Status, global};

use crate::{DefaultResponsePolicy, ZclResponseType};

/// Command matcher retained with pending and quarantined ZCL transactions.
#[derive(Clone, Copy, Debug)]
pub struct ResponseExpectation {
    matches_header: fn(Header) -> bool,
    request_command_id: u8,
    default_response_policy: DefaultResponsePolicy,
}

/// Outcome of matching a frame against a transaction's response policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseMatch {
    /// Deliver the frame through the caller's typed response conversion.
    Expected,

    /// Fail the transaction with the device's status, preserving unknown values.
    Rejected(Result<Status, u8>),

    /// A successful Default Response cannot supply the required command-specific result.
    UnexpectedDefault(u8),

    /// Route the frame normally without changing the transaction.
    Unrelated,
}

impl ResponseExpectation {
    /// Construct a matcher for a response type and the original request command ID.
    pub const fn new<T>(
        request_command_id: u8,
        default_response_policy: DefaultResponsePolicy,
    ) -> Self
    where
        T: ZclResponseType,
    {
        Self {
            matches_header: T::matches_response,
            request_command_id,
            default_response_policy,
        }
    }

    /// Classify a response, including error Default Responses to typed requests.
    pub fn classify(self, frame: &Frame<Cluster>) -> ResponseMatch {
        if let Cluster::Global(global::Command::DefaultResponse(response)) = frame.payload() {
            return self.classify_default(frame.header(), response);
        }
        if (self.matches_header)(frame.header()) {
            ResponseMatch::Expected
        } else {
            ResponseMatch::Unrelated
        }
    }

    /// Validate the original command before applying the request's Default Response policy.
    fn classify_default(self, header: Header, response: &DefaultResponse) -> ResponseMatch {
        if !DefaultResponse::matches_response(header)
            || response.command_id() != self.request_command_id
        {
            return ResponseMatch::Unrelated;
        }
        if self.default_response_policy == DefaultResponsePolicy::DefaultAllowed {
            return ResponseMatch::Expected;
        }
        let status = Status::try_from(response.status());
        if status == Ok(Status::Success) {
            ResponseMatch::UnexpectedDefault(self.request_command_id)
        } else {
            ResponseMatch::Rejected(status)
        }
    }
}

#[cfg(test)]
mod tests {
    use zb_core::Direction;
    use zb_zcl::global::default_response::DefaultResponse;
    use zb_zcl::global::read_attributes;
    use zb_zcl::{Command, Scope, Scoped};

    use super::{Cluster, Frame, Header, ResponseExpectation, ResponseMatch, ZclResponseType};

    const SEQUENCE: u8 = 7;

    struct ReadOrDefault;

    impl ZclResponseType for ReadOrDefault {
        fn matches_response(header: Header) -> bool {
            read_attributes::Response::matches_response(header)
                || DefaultResponse::matches_response(header)
        }
    }

    #[test]
    fn response_header_requires_scope_and_command_id() {
        let header = Header::new(
            Scope::ClusterSpecific,
            Direction::ServerToClient,
            false,
            None,
            SEQUENCE,
            <read_attributes::Response as Command>::ID,
        );
        assert!(!read_attributes::Response::matches_response(header));
        let header = Header::new(
            <read_attributes::Response as Scoped>::SCOPE,
            Direction::ServerToClient,
            false,
            None,
            SEQUENCE,
            <read_attributes::Response as Command>::ID,
        );
        assert!(read_attributes::Response::matches_response(header));
    }

    #[test]
    fn multiple_response_forms_still_validate_default_response_identity() {
        const CLUSTER_ID: u16 = 0x0006;
        const READ: [u8; 8] = [0x18, SEQUENCE, 0x01, 0x00, 0x00, 0x00, 0x10, 0x01];
        const DEFAULT: [u8; 5] = [0x18, SEQUENCE, 0x0b, 0x00, 0x86];
        const UNRELATED: [u8; 5] = [0x18, SEQUENCE, 0x0b, 0x01, 0x86];
        let expected = ResponseExpectation::new::<ReadOrDefault>(
            <read_attributes::Command as Command>::ID,
            crate::DefaultResponsePolicy::DefaultAllowed,
        );
        for bytes in [READ.as_slice(), DEFAULT.as_slice()] {
            let frame = Frame::<Cluster>::parse(CLUSTER_ID, bytes.iter().copied()).unwrap();
            assert_eq!(expected.classify(&frame), ResponseMatch::Expected);
        }
        let unrelated = Frame::<Cluster>::parse(CLUSTER_ID, UNRELATED.into_iter()).unwrap();
        assert_eq!(expected.classify(&unrelated), ResponseMatch::Unrelated);
    }

    #[test]
    fn explicitly_accepted_default_responses_remain_typed_for_every_status() {
        const CLUSTER_ID: u16 = 0x0006;
        const REQUEST_ID: u8 = <read_attributes::Command as Command>::ID;
        const DEFAULT_ID: u8 = <DefaultResponse as Command>::ID;
        const FRAME_CONTROL: u8 = 0x18;
        const UNKNOWN_STATUS: u8 = 0xff;
        for status in [
            zb_zcl::Status::Success as u8,
            zb_zcl::Status::Failure as u8,
            UNKNOWN_STATUS,
        ] {
            let frame = Frame::parse(
                CLUSTER_ID,
                [FRAME_CONTROL, SEQUENCE, DEFAULT_ID, REQUEST_ID, status].into_iter(),
            )
            .unwrap();
            assert_eq!(
                ResponseExpectation::new::<DefaultResponse>(
                    REQUEST_ID,
                    crate::DefaultResponsePolicy::DefaultAllowed
                )
                .classify(&frame),
                ResponseMatch::Expected
            );
            assert_eq!(
                ResponseExpectation::new::<ReadOrDefault>(
                    REQUEST_ID,
                    crate::DefaultResponsePolicy::DefaultAllowed
                )
                .classify(&frame),
                ResponseMatch::Expected
            );
        }
    }
}
