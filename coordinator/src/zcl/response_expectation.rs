use zb_zcl::{Cluster, Frame, Header, global};

use crate::ZclResponseType;

/// Command matcher retained with pending and quarantined ZCL transactions.
#[derive(Clone, Copy, Debug)]
pub struct ResponseExpectation {
    matches_header: fn(Header) -> bool,
    request_command_id: u8,
}

impl ResponseExpectation {
    /// Construct a matcher for a response type and the original request command ID.
    pub const fn new<T>(request_command_id: u8) -> Self
    where
        T: ZclResponseType,
    {
        Self {
            matches_header: T::matches_response,
            request_command_id,
        }
    }

    /// Validate the response command and, for Default Responses, its original command ID.
    pub fn matches(self, frame: &Frame<Cluster>) -> bool {
        if !(self.matches_header)(frame.header()) {
            return false;
        }
        if let Cluster::Global(global::Command::DefaultResponse(response)) = frame.payload() {
            return response.command_id() == self.request_command_id;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use zb_core::Direction;
    use zb_zcl::global::default_response::DefaultResponse;
    use zb_zcl::global::read_attributes;
    use zb_zcl::{Command, Scope, Scoped};

    use super::{Cluster, Frame, Header, ResponseExpectation, ZclResponseType};

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
        let expected =
            ResponseExpectation::new::<ReadOrDefault>(<read_attributes::Command as Command>::ID);
        for bytes in [READ.as_slice(), DEFAULT.as_slice()] {
            let frame = Frame::<Cluster>::parse(CLUSTER_ID, bytes.iter().copied()).unwrap();
            assert!(expected.matches(&frame));
        }
        let unrelated = Frame::<Cluster>::parse(CLUSTER_ID, UNRELATED.into_iter()).unwrap();
        assert!(!expected.matches(&unrelated));
    }
}
