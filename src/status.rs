#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Success,
    Failure,
    InvalidArguments,
    NoResponse,
    InvalidState,
    Unsupported,
    CommunicationError,
    BadResponse,
    NoResources,
    FatalError,
    NoNetwork,
}
