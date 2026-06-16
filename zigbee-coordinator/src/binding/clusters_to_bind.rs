use zigbee::{ClusterId, Endpoint};

use crate::binding::devices_ext::Endpoints;

pub trait ClustersToBind {
    fn clusters_to_bind(
        &self,
        clusters: &[ClusterId],
    ) -> impl Iterator<Item = (Endpoint, ClusterId)>;
}

impl ClustersToBind for Endpoints {
    fn clusters_to_bind(
        &self,
        clusters: &[ClusterId],
    ) -> impl Iterator<Item = (Endpoint, ClusterId)> {
        self.iter().flat_map(|(endpoint, (endpoint_info, _))| {
            clusters.iter().filter_map(move |cluster| {
                if endpoint_info
                    .descriptor()
                    .output_clusters()
                    .contains(&cluster.as_u16())
                {
                    Some((*endpoint, *cluster))
                } else {
                    None
                }
            })
        })
    }
}
