use std::collections::{BTreeMap, BTreeSet};

use zb_core::{Application, Cluster, Endpoint, FullAddress};
use zb_zdp::{BindReq, Destination, Status};

use crate::{Error, LocalNode, Zdp};

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
        src_endpoint_clusters: BTreeMap<Endpoint, BTreeSet<Cluster>>,
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

    /// Bind one source endpoint and cluster to the local coordinator endpoint.
    ///
    /// The destination is built from [`LocalNode::get_ieee_address`] and the default application
    /// endpoint. This is a convenience helper for common device-to-coordinator reporting bindings.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if reading the local IEEE address fails, the ZDP request fails, the
    /// response is invalid, or the response carries a non-success ZDP status.
    fn bind_to_self(
        &self,
        address: FullAddress,
        src_endpoint: Endpoint,
        cluster: Cluster,
    ) -> impl Future<Output = Result<(), Error>> + Send
    where
        Self: LocalNode + Sync,
    {
        async move {
            self.bind(
                address,
                src_endpoint,
                cluster,
                Destination::Extended {
                    address: self.get_ieee_address().await?,
                    endpoint: Application::default().into(),
                },
            )
            .await
        }
    }

    /// Bind multiple endpoint/cluster pairs to the local coordinator endpoint.
    ///
    /// The destination is built once from [`LocalNode::get_ieee_address`] and the default
    /// application endpoint. The returned map contains one result per source endpoint. If reading
    /// the local IEEE address fails, each endpoint is returned with the same error.
    fn bind_all_to_self(
        &self,
        address: FullAddress,
        src_endpoint_clusters: BTreeMap<Endpoint, BTreeSet<Cluster>>,
    ) -> impl Future<Output = BTreeMap<Endpoint, Result<(), Error>>> + Send
    where
        Self: LocalNode + Sync,
    {
        async move {
            let dst_address = match self.get_ieee_address().await {
                Ok(ieee_address) => ieee_address,
                Err(error) => {
                    return src_endpoint_clusters
                        .into_keys()
                        .map(|endpoint| (endpoint, Err(error.clone())))
                        .collect();
                }
            };

            self.bind_all(
                address,
                src_endpoint_clusters,
                Destination::Extended {
                    address: dst_address,
                    endpoint: Application::default().into(),
                },
            )
            .await
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
