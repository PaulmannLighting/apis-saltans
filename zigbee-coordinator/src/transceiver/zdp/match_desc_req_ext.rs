use zdp::{MatchDescReq, SimpleDescriptor};

/// Extension trait for `MatchDescReq`.
pub trait MatchDescReqExt {
    /// Return true if the given endpoint matches the request.
    fn matches(&self, endpoint: &SimpleDescriptor) -> bool;
}

impl MatchDescReqExt for MatchDescReq {
    fn matches(&self, endpoint: &SimpleDescriptor) -> bool {
        endpoint.profile_id() == self.profile_id()
            && self
                .in_cluster_list()
                .iter()
                .all(|cluster| endpoint.input_clusters().contains(cluster))
            && self
                .out_cluster_list()
                .iter()
                .all(|cluster| endpoint.output_clusters().contains(cluster))
    }
}
