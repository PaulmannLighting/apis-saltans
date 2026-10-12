use crate::{Cluster, Command, Header, Scoped};

/// Marker for commands whose only application response is a Default Response.
///
/// No command-specific payload can be converted into this uninhabited type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NoSpecificResponse {}

impl TryFrom<Cluster> for NoSpecificResponse {
    type Error = Cluster;

    fn try_from(response: Cluster) -> Result<Self, Self::Error> {
        Err(response)
    }
}

/// Defines which command headers can satisfy a typed ZCL response.
///
/// Individual command types implement this automatically through `Command` and `Scoped`.
/// Response enums accepting multiple command forms can implement this trait explicitly.
/// Implementations should match both scope and command ID and must not accept unrelated commands.
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

impl ZclResponseType for NoSpecificResponse {
    fn matches_response(_header: Header) -> bool {
        false
    }
}
