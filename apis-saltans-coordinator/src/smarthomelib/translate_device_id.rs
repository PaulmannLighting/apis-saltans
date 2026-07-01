use apis_saltans_hw::Ncp;
use smarthomelib::TranslateDeviceId;

use crate::Coordinator;

impl TranslateDeviceId<u16> for Coordinator {
    async fn translate_device_id(&self, id: u16) -> Result<Self::DeviceId, Self::Error> {
        self.ncp
            .short_id_to_ieee_address(id)
            .await
            .map_err(Into::into)
    }
}
