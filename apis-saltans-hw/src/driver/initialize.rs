use crate::common::{Error, NcpHandle};

/// Starts the command side of an NCP driver backend.
pub trait Initialize {
    /// Initialize the backend and return a handle for sending NCP commands.
    ///
    /// # Errors
    ///
    /// Returns an error if the backend cannot be initialized.
    fn init(self) -> impl Future<Output = Result<NcpHandle, Error>>;
}
