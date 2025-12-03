use aps::Command;
use le_stream::ToLeStream;

use crate::Error;

/// Network layer management entity (NLME) trait.
pub trait Nlme {
    /// The error type returned by NLME operations.
    type Error: std::error::Error;

    /// Send a unicast message.
    fn unicast_command<T>(
        &mut self,
        destination: u16,
        frame: Command<T>,
    ) -> impl Future<Output = Result<(), Error<Self::Error>>>
    where
        T: zcl::Command + ToLeStream;
}
