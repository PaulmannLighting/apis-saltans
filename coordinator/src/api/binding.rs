use std::collections::{BTreeMap, BTreeSet};

use zb_core::{Cluster, Endpoint, FullAddress};
use zb_zdp::{BindReq, Destination};

use crate::{Error, LocalNode, StatusExt, Zdp};

/// Trait for sending ZDP bind requests.
pub trait Binding {
    /// Bind one source endpoint and cluster to the given destination.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the ZDP request cannot be queued, transmission or reception fails,
    /// the response is invalid, or it completes with a non-success ZDP status.
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

    /// Bind matching remote endpoint output clusters to local coordinator endpoints.
    ///
    /// This method reads the coordinator IEEE address and local endpoint cluster sets through
    /// [`LocalNode`]. For each local endpoint, it intersects that endpoint's input clusters with
    /// every remote source endpoint's output clusters, then sends ZDP bind requests for the
    /// matching clusters only.
    ///
    /// The outer `Result` represents local coordinator lookup failures, such as failing to read the
    /// coordinator IEEE address or local endpoint cluster sets. The returned map contains per-source
    /// endpoint bind results for requests that were attempted.
    ///
    /// If several local endpoints can receive clusters from the same remote source endpoint, later
    /// local endpoint results overwrite earlier results for that source endpoint in the returned
    /// map.
    fn bind_all_to_self(
        &self,
        address: FullAddress,
        src_endpoint_clusters: BTreeMap<Endpoint, BTreeSet<Cluster>>,
    ) -> impl Future<Output = Result<BTreeMap<Endpoint, Result<(), Error>>, Error>> + Send
    where
        Self: LocalNode + Sync,
    {
        async move {
            let mut results = BTreeMap::new();
            let dst_address = self.get_ieee_address().await?;

            for (dst_endpoint, input_clusters) in self
                .get_endpoints()
                .await?
                .into_iter()
                .map(|(endpoint, clusters)| (endpoint, clusters.input().clone()))
            {
                let mut endpoint_clusters_to_bind = BTreeMap::new();

                for (src_endpoint, output_clusters) in &src_endpoint_clusters {
                    endpoint_clusters_to_bind.insert(
                        *src_endpoint,
                        input_clusters
                            .intersection(output_clusters)
                            .copied()
                            .collect(),
                    );
                }

                results.extend(
                    self.bind_all(
                        address,
                        endpoint_clusters_to_bind,
                        Destination::Extended {
                            address: dst_address,
                            endpoint: dst_endpoint.into(),
                        },
                    )
                    .await,
                );
            }

            Ok(results)
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
        self.communicate(
            address.short_id(),
            BindReq::new(
                address.ieee_address(),
                endpoint,
                cluster.into(),
                destination,
            ),
        )
        .await?
        .await?
        .status()
        .ensure_success()
    }
}
