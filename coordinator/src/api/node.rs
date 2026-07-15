use zb_core::node::Descriptor;
use zb_core::short_id::Device;
use zb_core::types::tlv::FragmentationParameters;
use zb_zdp::NodeDescReq;

use crate::Error;
use crate::api::Zdp;

pub trait Node {
    fn descriptor(
        &self,
        device: Device,
        fragmentation: Option<FragmentationParameters>,
    ) -> impl Future<Output = Result<Descriptor, Error>> + Send;
}

impl<T> Node for T
where
    T: Zdp + Sync,
{
    async fn descriptor(
        &self,
        device: Device,
        fragmentation: Option<FragmentationParameters>,
    ) -> Result<Descriptor, Error> {
        let fragmentation = fragmentation
            .unwrap_or_else(|| FragmentationParameters::new(device.into(), None, None));

        let response = self
            .communicate(device, NodeDescReq::from(fragmentation))
            .await?;

        Ok(response.try_into()?)
    }
}
