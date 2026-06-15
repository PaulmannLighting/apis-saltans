use zdp::SimpleDescriptor;
use zigbee::{Application, ClusterId, Endpoint};

use crate::discovery::EndpointInfo;

/// Helper trait to filter out application endpoints that have the Basic cluster.
pub trait ApplicationEndpointsWithBasicCluster<T> {
    /// Filter out application endpoints that have the Basic cluster.
    ///
    /// This is intended to be used with the [`Iterator::filter_map`] method.
    fn filter(self) -> Option<(Application, T)>;
}

impl<'a> ApplicationEndpointsWithBasicCluster<&'a SimpleDescriptor>
    for (&'a Endpoint, &'a SimpleDescriptor)
{
    fn filter(self) -> Option<(Application, &'a SimpleDescriptor)> {
        if let Endpoint::Application(application) = self.0
            && self.1.input_clusters().contains(&ClusterId::Basic.into())
        {
            Some((*application, self.1))
        } else {
            None
        }
    }
}

impl<'a> ApplicationEndpointsWithBasicCluster<&'a EndpointInfo>
    for (&'a Endpoint, &'a EndpointInfo)
{
    fn filter(self) -> Option<(Application, &'a EndpointInfo)> {
        if let Endpoint::Application(application) = self.0
            && self
                .1
                .descriptor()
                .input_clusters()
                .contains(&ClusterId::Basic.into())
        {
            Some((*application, self.1))
        } else {
            None
        }
    }
}
