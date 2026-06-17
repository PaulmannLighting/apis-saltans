use std::sync::Arc;

/// Persistence API errors.

#[derive(Debug)]
pub enum Error {
    /// Send error.
    Send,

    /// Receive error.
    Receive,

    /// IO error.
    Io(Arc<std::io::Error>),

    /// Storage error.
    Store(Option<Arc<dyn std::error::Error + Send + Sync + 'static>>),

    /// Loading error.
    Load(Option<Arc<dyn std::error::Error + Send + Sync + 'static>>),
}
