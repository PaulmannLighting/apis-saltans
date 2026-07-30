use std::time::Duration;

pub use self::key::Key;
pub use self::lifecycle::{Cancellation, Token};
pub use self::registry::Registry;

mod key;
mod lifecycle;
mod registry;

/// Maximum time retained for a pending ZCL or ZDP response.
pub const PROTOCOL_RESPONSE_TIMEOUT: Duration = Duration::from_secs(30);
/// Maximum additional time retained for a late ZCL or ZDP response.
pub const PROTOCOL_QUARANTINE_TIMEOUT: Duration = Duration::from_secs(30);
