use std::collections::{BTreeMap, BTreeSet};

use zb_core::Endpoint;
use zb_core::short_id::Device;
use zb_zdp::{ActiveEpReq, SimpleDescReq, SimpleDescriptor, Status};

use crate::Error;
use crate::api::Zdp;

pub trait Endpoints {
    fn endpoints(
        &self,
        device: Device,
    ) -> impl Future<Output = Result<BTreeSet<Endpoint>, Error>> + Send;

    fn descriptor(
        &self,
        device: Device,
        endpoint: Endpoint,
    ) -> impl Future<Output = Result<Option<SimpleDescriptor>, Error>> + Send;

    fn descriptors(
        &self,
        device: Device,
        endpoints: BTreeSet<Endpoint>,
    ) -> impl Future<Output = BTreeMap<Endpoint, Result<Option<SimpleDescriptor>, Error>>> + Send
    where
        Self: Sync,
    {
        async move {
            let mut results = BTreeMap::new();

            for endpoint in endpoints {
                results.insert(endpoint, self.descriptor(device, endpoint).await);
            }

            results
        }
    }
}

impl<T> Endpoints for T
where
    T: Zdp + Sync,
{
    async fn endpoints(&self, device: Device) -> Result<BTreeSet<Endpoint>, Error> {
        let response = self
            .communicate(device, ActiveEpReq::new(device.into()))
            .await?;

        let status = response.status();

        if Ok(Status::Success) == status {
            return Ok(response.into_active_eps().filter_map(Result::ok).collect());
        }

        Err(status.into())
    }

    async fn descriptor(
        &self,
        device: Device,
        endpoint: Endpoint,
    ) -> Result<Option<SimpleDescriptor>, Error> {
        let response = self
            .communicate(device, SimpleDescReq::new(device.into(), endpoint))
            .await?;

        let status = response.status();

        if status == Ok(Status::Success) {
            return Ok(response.into_descriptor());
        }

        Err(status.into())
    }
}
