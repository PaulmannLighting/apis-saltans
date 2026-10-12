use std::collections::{BTreeMap, BTreeSet};

use zb_core::endpoint::Application;
use zb_core::{Cluster, Endpoint, FullAddress};
use zb_zdp::{BindReq, Destination, SimpleDescriptor};

use crate::{Error, LocalNode, StatusExt, Zdp};

/// Outcome of one attempted binding, including its complete endpoint and cluster identity.
#[derive(Debug)]
pub struct BindingResult {
    /// Remote endpoint producing the cluster's messages.
    pub source_endpoint: Endpoint,
    /// Cluster bound by this attempt.
    pub cluster: Cluster,
    /// Destination of the binding.
    pub destination: Destination,
    /// Success or failure of this individual attempt.
    pub result: Result<(), Error>,
}

/// Bindings selected for one local application endpoint.
struct EndpointBindings {
    destination: Application,
    source_clusters: BTreeMap<Endpoint, BTreeSet<Cluster>>,
}

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
    /// Returns one identified result per attempted binding, in endpoint and cluster order.
    /// A later success never overwrites an earlier failure.
    fn bind_all(
        &self,
        address: FullAddress,
        src_endpoint_clusters: BTreeMap<Endpoint, BTreeSet<Cluster>>,
        destination: Destination,
    ) -> impl Future<Output = Vec<BindingResult>> + Send
    where
        Self: Sync,
    {
        async move {
            let mut results = Vec::new();

            for (endpoint, clusters) in src_endpoint_clusters {
                for cluster in clusters {
                    results.push(BindingResult {
                        source_endpoint: endpoint,
                        cluster,
                        destination,
                        result: self.bind(address, endpoint, cluster, destination).await,
                    });
                }
            }

            results
        }
    }

    /// Bind matching remote endpoint output clusters to local coordinator endpoints.
    ///
    /// This method reads the coordinator IEEE address and local simple descriptors through
    /// [`LocalNode`]. For each local endpoint, it intersects that descriptor's input clusters with
    /// every remote source endpoint's output clusters, then sends ZDP bind requests for the
    /// matching clusters only.
    ///
    /// The outer `Result` represents local coordinator lookup failures, such as failing to read the
    /// coordinator IEEE address or local endpoint descriptors. Each attempted binding retains its
    /// source endpoint, cluster, destination, and result. Results follow local descriptor order,
    /// then source endpoint and cluster order.
    fn bind_all_to_self(
        &self,
        address: FullAddress,
        src_endpoint_clusters: BTreeMap<Endpoint, BTreeSet<Cluster>>,
    ) -> impl Future<Output = Result<Vec<BindingResult>, Error>> + Send
    where
        Self: LocalNode + Sync,
    {
        async move {
            let mut results = Vec::new();
            let dst_address = self.get_ieee_address().await?;

            let descriptors = self.get_endpoints().await?;
            for plan in binding_plan(&descriptors, &src_endpoint_clusters) {
                results.extend(
                    self.bind_all(
                        address,
                        plan.source_clusters,
                        Destination::Extended {
                            address: dst_address,
                            endpoint: plan.destination.into(),
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

/// Select matching clusters without performing I/O, retaining local descriptor order.
fn binding_plan(
    descriptors: &[SimpleDescriptor],
    sources: &BTreeMap<Endpoint, BTreeSet<Cluster>>,
) -> Vec<EndpointBindings> {
    descriptors
        .iter()
        .filter_map(|descriptor| {
            let destination = application_endpoint(descriptor)?;
            let inputs: BTreeSet<_> = descriptor
                .input_clusters()
                .iter()
                .copied()
                .filter_map(|cluster| Cluster::try_from(cluster).ok())
                .collect();
            let source_clusters = sources
                .iter()
                .map(|(endpoint, outputs)| {
                    (*endpoint, inputs.intersection(outputs).copied().collect())
                })
                .collect();
            Some(EndpointBindings {
                destination,
                source_clusters,
            })
        })
        .collect()
}

/// Return the application endpoint declared by a local simple descriptor.
fn application_endpoint(descriptor: &SimpleDescriptor) -> Option<Application> {
    let Ok(Endpoint::Application(endpoint)) = descriptor.endpoint() else {
        return None;
    };
    Some(endpoint)
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use tokio::runtime::Runtime;
    use zb_core::endpoint::Application;
    use zb_core::short_id::Device;
    use zb_core::{Cluster, Endpoint, FullAddress, IeeeAddress, Profile};
    use zb_zdp::{AppFlags, Clusters, Destination, SimpleDescriptor};

    use super::{Binding, application_endpoint, binding_plan};
    use crate::{Error, LocalNode};

    const DECLARED_ENDPOINT: u8 = 0x0B;
    const DEVICE_ID: u16 = 0x0100;

    const ADDRESS: FullAddress = FullAddress::new(
        IeeeAddress::new(1, 2, 3, 4, 5, 6, 7, 8),
        Device::new(0x1234).expect("valid device address"),
    );

    struct TestBindings;

    impl Binding for TestBindings {
        async fn bind(
            &self,
            _: FullAddress,
            _: Endpoint,
            cluster: Cluster,
            _: Destination,
        ) -> Result<(), Error> {
            if cluster == Cluster::OnOff {
                Err(Error::ProtocolResponseTimeout)
            } else {
                Ok(())
            }
        }
    }

    impl LocalNode for TestBindings {
        async fn get_endpoints(&self) -> Result<Box<[SimpleDescriptor]>, Error> {
            Ok([
                Endpoint::Application(Application::MIN),
                Endpoint::try_from(DECLARED_ENDPOINT).unwrap(),
            ]
            .into_iter()
            .map(|endpoint| {
                descriptor_with_inputs(
                    endpoint,
                    &[Cluster::OnOff.as_u16(), Cluster::Level.as_u16()],
                )
            })
            .collect())
        }
        async fn get_pan_id(&self) -> Result<u16, Error> {
            Ok(DEVICE_ID)
        }
        async fn get_ieee_address(&self) -> Result<IeeeAddress, Error> {
            Ok(ADDRESS.ieee_address())
        }
    }

    #[test]
    fn batch_results_preserve_failures_across_clusters_and_destinations() {
        Runtime::new().unwrap().block_on(async {
            let endpoint = Endpoint::Application(Application::MIN);
            let sources =
                BTreeMap::from([(endpoint, BTreeSet::from([Cluster::OnOff, Cluster::Level]))]);
            let results = TestBindings
                .bind_all_to_self(ADDRESS, sources)
                .await
                .unwrap();
            assert_eq!(results.len(), 4);
            for pair in results.chunks_exact(2) {
                assert_eq!(pair[0].source_endpoint, endpoint);
                assert_eq!(pair[0].cluster, Cluster::OnOff);
                assert!(matches!(
                    pair[0].result,
                    Err(Error::ProtocolResponseTimeout)
                ));
                assert_eq!(pair[1].cluster, Cluster::Level);
                assert!(pair[1].result.is_ok());
                assert_eq!(pair[0].destination, pair[1].destination);
            }
            assert_ne!(results[0].destination, results[2].destination);
        });
    }

    #[test]
    fn uses_endpoint_declared_by_descriptor() {
        let descriptor =
            descriptor(Endpoint::try_from(DECLARED_ENDPOINT).expect("declared endpoint is valid"));

        assert_eq!(
            application_endpoint(&descriptor),
            Application::new(DECLARED_ENDPOINT)
        );
    }

    #[test]
    fn ignores_non_application_descriptor_endpoint() {
        let descriptor = descriptor(Endpoint::Data);

        assert_eq!(application_endpoint(&descriptor), None);
    }

    #[test]
    fn selects_cluster_intersections_in_descriptor_order() {
        const UNKNOWN_CLUSTER: u16 = 0xffff;
        let first = Endpoint::Application(Application::MIN);
        let second = Endpoint::try_from(DECLARED_ENDPOINT).expect("valid endpoint");
        let sources = BTreeMap::from([
            (first, BTreeSet::from([Cluster::OnOff, Cluster::Level])),
            (second, BTreeSet::from([Cluster::Scenes])),
        ]);
        let descriptors = [
            descriptor_with_inputs(second, &[Cluster::OnOff.as_u16(), UNKNOWN_CLUSTER]),
            descriptor_with_inputs(Endpoint::Data, &[Cluster::OnOff.as_u16()]),
            descriptor_with_inputs(first, &[Cluster::Level.as_u16()]),
        ];
        let plan = binding_plan(&descriptors, &sources);
        assert_eq!(plan.len(), 2);
        assert_eq!(Endpoint::from(plan[0].destination), second);
        assert_eq!(Endpoint::from(plan[1].destination), first);
        assert_eq!(
            plan[0].source_clusters[&first],
            BTreeSet::from([Cluster::OnOff])
        );
        assert_eq!(
            plan[1].source_clusters[&first],
            BTreeSet::from([Cluster::Level])
        );
        assert!(plan[0].source_clusters[&second].is_empty());
        assert!(plan[1].source_clusters[&second].is_empty());
    }

    fn descriptor_with_inputs(endpoint: Endpoint, inputs: &[u16]) -> SimpleDescriptor {
        let mut clusters = Clusters::new();
        for cluster in inputs {
            clusters
                .push(*cluster)
                .expect("test clusters fit descriptor");
        }
        SimpleDescriptor::new(
            endpoint,
            Profile::ZigbeeHomeAutomation,
            DEVICE_ID,
            AppFlags::empty(),
            clusters,
            Clusters::new(),
        )
    }

    fn descriptor(endpoint: Endpoint) -> SimpleDescriptor {
        SimpleDescriptor::new(
            endpoint,
            Profile::ZigbeeHomeAutomation,
            DEVICE_ID,
            AppFlags::empty(),
            Clusters::new(),
            Clusters::new(),
        )
    }
}
