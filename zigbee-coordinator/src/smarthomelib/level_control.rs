use std::time::Duration;

use smarthomelib::LevelControl;

use crate::{Coordinator, Error};

impl LevelControl for Coordinator {
    async fn move_to_level(
        &self,
        _device: Self::DeviceId,
        _endpoint: Self::EndpointId,
        _level: u8,
        _transition: Option<Duration>,
    ) -> Result<(), Self::Error> {
        Err(Error::Unsupported("level control"))
    }
}
