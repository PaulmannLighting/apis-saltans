use std::collections::BTreeMap;

use zb_core::{Cluster, Endpoint, FullAddress};
use zb_zdp::{BindReq, Destination, Status};

use crate::{Error, Zdp};

/// Trait for sending ZDP bind requests.
pub trait Binding {
    /// Bind one source endpoint and cluster to the given destination.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the ZDP request fails, times out, returns an invalid response, or
    /// completes with a non-success ZDP status.
    fn bind(
        &self,
        address: FullAddress,
        src_endpoint: Endpoint,
        cluster: Cluster,
        destination: Destination,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Bind multiple endpoint/cluster pairs to the same destination.
    ///
    /// The returned map contains one result per source endpoint. If an endpoint has multiple
    /// clusters, the last cluster result for that endpoint is stored.
    fn bind_all(
        &self,
        address: FullAddress,
        src_endpoint_clusters: BTreeMap<Endpoint, Box<[Cluster]>>,
        destination: Destination,
    ) -> impl Future<Output = BTreeMap<Endpoint, Result<(), Error>>> + Send
    where
        Self: Sync,
    {
        async move {
            let mut results = BTreeMap::new();

            for (endpoint, clusters) in src_endpoint_clusters {
                for cluster in clusters {
                    results.insert(
                        endpoint,
                        self.bind(address, endpoint, cluster, destination).await,
                    );
                }
            }

            results
        }
    }
}

impl<T> Binding for T
where
    T: Zdp + Sync,
{
    async fn bind(
        &self,
        address: FullAddress,
        endpoint: Endpoint,
        cluster: Cluster,
        destination: Destination,
    ) -> Result<(), Error> {
        let response = self
            .communicate(
                address.short_id(),
                BindReq::new(
                    address.ieee_address(),
                    endpoint,
                    cluster.into(),
                    destination,
                ),
            )
            .await?;

        let status = response.status();

        if Ok(Status::Success) == status {
            return Ok(());
        }

        Err(status.into())
    }
}
