use std::collections::{BTreeMap, BTreeSet};

use zb_core::Endpoint;
use zb_core::short_id::Device;
pub use zb_zdp::SimpleDescriptor;
use zb_zdp::{ActiveEpReq, SimpleDescReq, Status};

use crate::Error;
use crate::api::Zdp;

/// Trait for discovering active endpoints and simple descriptors.
pub trait Endpoints {
    /// Read the active application endpoints from a device.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if communication fails, the response is invalid, or the ZDP status is
    /// not successful.
    fn endpoints(
        &self,
        device: Device,
    ) -> impl Future<Output = Result<BTreeSet<Endpoint>, Error>> + Send;

    /// Read a simple descriptor for one endpoint.
    ///
    /// A successful response without a descriptor is returned as `Ok(None)`.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if communication fails, the response is invalid, or the ZDP status is
    /// not successful.
    fn descriptor(
        &self,
        device: Device,
        endpoint: Endpoint,
    ) -> impl Future<Output = Result<Option<SimpleDescriptor>, Error>> + Send;

    /// Read simple descriptors for all active endpoints on a device.
    ///
    /// This first calls [`Self::endpoints`] to discover the active endpoint set. If endpoint
    /// discovery fails, the outer `Result` contains that error and no descriptor requests are sent.
    ///
    /// If endpoint discovery succeeds, each endpoint receives its own descriptor result in the
    /// returned map. This lets callers keep partial descriptor discovery results when one endpoint
    /// fails but others succeed.
    fn descriptors(
        &self,
        device: Device,
    ) -> impl Future<
        Output = Result<BTreeMap<Endpoint, Result<Option<SimpleDescriptor>, Error>>, Error>,
    > + Send
    where
        Self: Sync,
    {
        async move {
            let mut results = BTreeMap::new();

            for endpoint in self.endpoints(device).await? {
                results.insert(endpoint, self.descriptor(device, endpoint).await);
            }

            Ok(results)
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
