use smarthomelib::{DeviceInventory, EndpointCapabilities, ProtocolDevice, ProtocolEndpoint};
use zigbee::ClusterId;

use crate::{Coordinator, NetworkManager};

impl DeviceInventory for Coordinator {
    async fn devices(
        &self,
    ) -> Result<Vec<ProtocolDevice<Self::DeviceId, Self::EndpointId>>, Self::Error> {
        let state = self.state().await?;
        Ok(state
            .into_iter()
            .map(|(id, device)| {
                let endpoints = device
                    .endpoints
                    .into_iter()
                    .map(|(id, endpoint)| {
                        let descriptor = endpoint.descriptor();
                        ProtocolEndpoint::new(id, capabilities(descriptor.input_clusters()))
                    })
                    .collect();
                ProtocolDevice::new(id, endpoints)
            })
            .collect())
    }
}

fn capabilities(input_clusters: &[u16]) -> EndpointCapabilities {
    EndpointCapabilities::new(
        has_cluster(input_clusters, ClusterId::OnOff),
        false,
        has_cluster(input_clusters, ClusterId::ColorControl),
    )
}

fn has_cluster(clusters: &[u16], cluster_id: ClusterId) -> bool {
    clusters.contains(&cluster_id.as_u16())
}

// Inline tests keep the private cluster-to-capability mapping private; constructing a public
// Coordinator inventory fixture would require real coordinator state wiring.
#[cfg(test)]
mod tests {
    use smarthomelib::EndpointCapabilities;
    use zigbee::ClusterId;

    use super::capabilities;

    #[test]
    fn capabilities_when_known_input_clusters_are_present_then_advertises_supported_capabilities() {
        let capabilities = capabilities(&[
            ClusterId::OnOff.as_u16(),
            ClusterId::Level.as_u16(),
            ClusterId::ColorControl.as_u16(),
        ]);

        assert_eq!(capabilities, EndpointCapabilities::new(true, false, true));
    }

    #[test]
    fn capabilities_when_clusters_are_missing_then_does_not_advertise_capabilities() {
        let capabilities = capabilities(&[
            ClusterId::Basic.as_u16(),
            ClusterId::Identify.as_u16(),
            ClusterId::Groups.as_u16(),
        ]);

        assert_eq!(capabilities, EndpointCapabilities::default());
    }
}
