use crate::global::default_response::DefaultResponse;
use crate::{Cluster, Command, Header, Scoped, global};

/// A response that permits either a Default Response or a command-specific result.
///
/// Default Responses preserve all status values, including failures and unknown statuses.
/// Use a concrete response type when a Default Response cannot supply the required result,
/// or [`DefaultResponse`] when only a Default Response is expected.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MaybeDefault<T> {
    /// A Default Response; inspect its status to determine device success.
    Default(DefaultResponse),

    /// A command-specific response.
    Specific(T),
}

impl<T> TryFrom<Cluster> for MaybeDefault<T>
where
    T: TryFrom<Cluster>,
{
    type Error = T::Error;

    fn try_from(response: Cluster) -> Result<Self, Self::Error> {
        match response {
            Cluster::Global(global::Command::DefaultResponse(response)) => {
                Ok(Self::Default(*response))
            }
            response => T::try_from(response).map(Self::Specific),
        }
    }
}

impl<T> ZclResponseType for MaybeDefault<T>
where
    T: ZclResponseType,
{
    fn matches_response(header: Header) -> bool {
        DefaultResponse::matches_response(header) || T::matches_response(header)
    }
}

/// Defines which command headers can satisfy a typed ZCL response.
///
/// Individual command types implement this automatically through `Command` and `Scoped`.
/// Response enums accepting multiple command forms can implement this trait explicitly.
/// Implementations should match both scope and command ID and must not accept unrelated commands.
/// Accepting a Default Response header opts into receiving all its status values as response data.
/// Transaction correlation separately validates the original command ID carried in its payload.
pub trait ZclResponseType {
    /// Whether this header describes an accepted response command.
    fn matches_response(header: Header) -> bool;
}

impl<T> ZclResponseType for T
where
    T: Command + Scoped,
{
    fn matches_response(header: Header) -> bool {
        header.control().typ() == Ok(T::SCOPE) && header.command_id() == T::ID
    }
}

#[cfg(test)]
mod tests {
    use super::{Cluster, MaybeDefault, ZclResponseType};
    use crate::global::read_attributes::Response;
    use crate::{Frame, Status};

    const CLUSTER_ID: u16 = 0x0006;
    const SEQUENCE: u8 = 7;
    const REQUEST_ID: u8 = 0x00;
    const FRAME_CONTROL: u8 = 0x18;
    const DEFAULT_ID: u8 = 0x0b;
    const UNKNOWN_STATUS: u8 = 0xff;
    const READ: [u8; 8] = [FRAME_CONTROL, SEQUENCE, 0x01, 0x00, 0x00, 0x00, 0x10, 0x01];
    const REPORT: [u8; 7] = [FRAME_CONTROL, SEQUENCE, 0x0a, 0x00, 0x00, 0x10, 0x01];

    #[test]
    fn default_responses_preserve_every_status() {
        for status in [Status::Success as u8, Status::Failure as u8, UNKNOWN_STATUS] {
            let frame = Frame::<Cluster>::parse(
                CLUSTER_ID,
                [FRAME_CONTROL, SEQUENCE, DEFAULT_ID, REQUEST_ID, status].into_iter(),
            )
            .unwrap();
            assert!(MaybeDefault::<Response>::matches_response(frame.header()));
            assert!(!Response::matches_response(frame.header()));
            let MaybeDefault::Default(response) =
                MaybeDefault::<Response>::try_from(frame.into_payload()).unwrap()
            else {
                panic!("expected a Default Response");
            };
            assert_eq!(response.command_id(), REQUEST_ID);
            assert_eq!(response.status(), status);
        }
    }

    #[test]
    fn specific_response_preserves_payload() {
        let frame = Frame::<Cluster>::parse(CLUSTER_ID, READ.into_iter()).unwrap();
        assert!(MaybeDefault::<Response>::matches_response(frame.header()));
        let expected = Response::try_from(frame.payload().clone()).unwrap();
        assert_eq!(
            MaybeDefault::<Response>::try_from(frame.into_payload()).unwrap(),
            MaybeDefault::Specific(expected)
        );
    }

    #[test]
    fn unrelated_response_is_neither_matched_nor_converted() {
        let frame = Frame::<Cluster>::parse(CLUSTER_ID, REPORT.into_iter()).unwrap();
        assert!(!MaybeDefault::<Response>::matches_response(frame.header()));
        let payload = frame.into_payload();
        assert_eq!(
            MaybeDefault::<Response>::try_from(payload.clone()),
            Err(payload)
        );
    }
}
