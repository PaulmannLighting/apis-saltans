use std::num::NonZero;
use std::time::Duration;

use const_env::env_lit;

pub use self::key::Key;
pub use self::lifecycle::{Cancellation, Token};
pub use self::registry::Registry;

mod key;
mod lifecycle;
mod registry;

const PROTOCOL_RESPONSE_TIMEOUT_SECS: NonZero<u64> = NonZero::new(env_lit!(
    "ZIGBEE_COORDINATOR_PROTOCOL_RESPONSE_TIMEOUT_SECS",
    30
))
.expect("ZIGBEE_COORDINATOR_PROTOCOL_RESPONSE_TIMEOUT_SECS must be greater than zero");
const PROTOCOL_QUARANTINE_TIMEOUT_SECS: NonZero<u64> = NonZero::new(env_lit!(
    "ZIGBEE_COORDINATOR_PROTOCOL_QUARANTINE_TIMEOUT_SECS",
    30
))
.expect("ZIGBEE_COORDINATOR_PROTOCOL_QUARANTINE_TIMEOUT_SECS must be greater than zero");

/// Maximum time retained for a pending ZCL or ZDP response.
pub const PROTOCOL_RESPONSE_TIMEOUT: Duration =
    Duration::from_secs(PROTOCOL_RESPONSE_TIMEOUT_SECS.get());
/// Maximum additional time retained for a late ZCL or ZDP response.
pub const PROTOCOL_QUARANTINE_TIMEOUT: Duration =
    Duration::from_secs(PROTOCOL_QUARANTINE_TIMEOUT_SECS.get());
