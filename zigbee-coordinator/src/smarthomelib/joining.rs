use std::time::Duration;

use smarthomelib::ZigbeeJoining;

use crate::Coordinator;

impl ZigbeeJoining for Coordinator {
    async fn allow_joining(&self, duration: Duration) -> Result<Duration, Self::Error> {
        crate::Joining::allow_joining(self, duration).await
    }
}
