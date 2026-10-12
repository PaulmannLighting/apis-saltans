use std::any::type_name;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::marker::PhantomData;

use bytes::Bytes;
use zb_zcl::global::default_response::DefaultResponse;
use zb_zcl::{Cluster, Frame, Header, Status, UnsequencedFrame, global};

use crate::{DefaultResponsePolicy, ZclRequestPolicy, ZclResponseType};

/// Erased response contract retained with pending and quarantined ZCL transactions.
///
/// A boxed adapter retains the response type's matching behavior together with the request
/// command identity and Default Response policy.
#[derive(Debug)]
pub struct ResponseExpectation {
    contract: Box<dyn ResponseContract>,
}

/// Object-safe classification shared by typed and encoded response contracts.
trait ResponseContract
where
    Self: Debug + Send + Sync,
{
    /// Classify a frame without consuming the contract needed for subsequent or late replies.
    fn classify(&self, frame: &Frame<Cluster>) -> ResponseMatch;
}

/// Response type adapter retaining a request's command identity and policy.
struct ResponseTypeContract<T> {
    response: PhantomData<T>,
    request_command_id: u8,
    default_response_policy: DefaultResponsePolicy,
}

impl<T> Debug for ResponseTypeContract<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        formatter
            .debug_struct("ResponseTypeContract")
            .field("response", &type_name::<T>())
            .field("request_command_id", &self.request_command_id)
            .field("default_response_policy", &self.default_response_policy)
            .finish()
    }
}

impl<T> ResponseContract for ResponseTypeContract<T>
where
    T: ZclResponseType + Send + Sync,
{
    fn classify(&self, frame: &Frame<Cluster>) -> ResponseMatch {
        classify_response::<T>(frame, self.request_command_id, self.default_response_policy)
    }
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
    /// Erase a request type's fixed response contract for storage in the actor registry.
    pub(crate) fn for_request<C>() -> Self
    where
        C: ZclRequestPolicy,
        C::Response: ZclResponseType + Send + Sync + 'static,
    {
        Self {
            contract: Box::new(ResponseTypeContract::<C::Response> {
                response: PhantomData,
                request_command_id: C::ID,
                default_response_policy: C::DEFAULT_RESPONSE_POLICY,
            }),
        }
    }

    /// Build a raw contract, taking the original command ID from the encoded frame.
    ///
    /// The caller is responsible for selecting the appropriate response type and policy.
    pub(crate) fn for_encoded<T>(
        request: &UnsequencedFrame<Bytes>,
        default_response_policy: DefaultResponsePolicy,
    ) -> Self
    where
        T: ZclResponseType + Send + Sync + 'static,
    {
        Self {
            contract: Box::new(ResponseTypeContract::<T> {
                response: PhantomData,
                request_command_id: request.header().command_id(),
                default_response_policy,
            }),
        }
    }

    /// Classify a response using the stored request contract.
    pub(crate) fn classify(&self, frame: &Frame<Cluster>) -> ResponseMatch {
        self.contract.classify(frame)
    }
}

/// Apply Default Response policy before matching the expected response type's header.
fn classify_response<T>(
    frame: &Frame<Cluster>,
    request_command_id: u8,
    policy: DefaultResponsePolicy,
) -> ResponseMatch
where
    T: ZclResponseType,
{
    if let Cluster::Global(global::Command::DefaultResponse(response)) = frame.payload() {
        return classify_default(frame.header(), response, request_command_id, policy);
    }
    if T::matches_response(frame.header()) {
        ResponseMatch::Expected
    } else {
        ResponseMatch::Unrelated
    }
}

/// Validate the original command before applying the request's Default Response policy.
fn classify_default(
    header: Header,
    response: &DefaultResponse,
    request_command_id: u8,
    policy: DefaultResponsePolicy,
) -> ResponseMatch {
    if !DefaultResponse::matches_response(header) || response.command_id() != request_command_id {
        return ResponseMatch::Unrelated;
    }
    if policy == DefaultResponsePolicy::DefaultAllowed {
        return ResponseMatch::Expected;
    }
    let status = Status::try_from(response.status());
    if status == Ok(Status::Success) {
        ResponseMatch::UnexpectedDefault(request_command_id)
    } else {
        ResponseMatch::Rejected(status)
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
        let expected = ResponseExpectation::for_encoded::<ReadOrDefault>(
            &encoded_request(<read_attributes::Command as Command>::ID),
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
                ResponseExpectation::for_encoded::<DefaultResponse>(
                    &encoded_request(REQUEST_ID),
                    crate::DefaultResponsePolicy::DefaultAllowed
                )
                .classify(&frame),
                ResponseMatch::Expected
            );
            assert_eq!(
                ResponseExpectation::for_encoded::<ReadOrDefault>(
                    &encoded_request(REQUEST_ID),
                    crate::DefaultResponsePolicy::DefaultAllowed
                )
                .classify(&frame),
                ResponseMatch::Expected
            );
        }
    }
    #[test]
    fn typed_read_contract_matches_specific_reply_and_terminates_on_defaults() {
        const CLUSTER_ID: u16 = 0x0006;
        const READ: [u8; 8] = [0x18, SEQUENCE, 0x01, 0x00, 0x00, 0x00, 0x10, 0x01];
        const REPORT: [u8; 7] = [0x18, SEQUENCE, 0x0a, 0x00, 0x00, 0x10, 0x01];
        const UNKNOWN: u8 = 0xff;
        const DEFAULT_ID: u8 = <DefaultResponse as Command>::ID;
        const REQUEST_ID: u8 = <read_attributes::Command as Command>::ID;
        const FRAME_CONTROL: u8 = 0x18;
        let expectation = ResponseExpectation::for_request::<read_attributes::Command>();
        assert_eq!(
            expectation.classify(&Frame::parse(CLUSTER_ID, READ.into_iter()).unwrap()),
            ResponseMatch::Expected
        );
        assert_eq!(
            expectation.classify(&Frame::parse(CLUSTER_ID, REPORT.into_iter()).unwrap()),
            ResponseMatch::Unrelated
        );
        for (status, expected) in [
            (
                super::Status::Success as u8,
                ResponseMatch::UnexpectedDefault(REQUEST_ID),
            ),
            (
                super::Status::Failure as u8,
                ResponseMatch::Rejected(Ok(super::Status::Failure)),
            ),
            (UNKNOWN, ResponseMatch::Rejected(Err(UNKNOWN))),
        ] {
            let frame = Frame::parse(
                CLUSTER_ID,
                [FRAME_CONTROL, SEQUENCE, DEFAULT_ID, REQUEST_ID, status].into_iter(),
            )
            .unwrap();
            assert_eq!(expectation.classify(&frame), expected);
        }
    }

    fn encoded_request(command_id: u8) -> zb_zcl::UnsequencedFrame<bytes::Bytes> {
        zb_zcl::UnsequencedFrame::new(
            zb_zcl::UnsequencedHeader::new(
                Scope::Global,
                Direction::ClientToServer,
                false,
                None,
                command_id,
            ),
            bytes::Bytes::new(),
        )
    }
}
