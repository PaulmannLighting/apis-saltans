use std::collections::BTreeMap;

use zdp::SimpleDescriptor;
use zigbee::{Address, Application, ClusterId, Endpoint};

use super::endpoint_info::EndpointInfo;

/// Type alias for a map of devices to their endpoints.
pub type Devices = BTreeMap<Address, BTreeMap<Endpoint, EndpointInfo>>;

/// Helper trait to filter out application endpoints that have the Basic cluster.
pub trait DevicesExt<T> {
    /// Filter out application endpoints that have the Basic cluster.
    ///
    /// This is intended to be used with the [`Iterator::filter_map`] method.
    fn application_eps_with_basic_cluster(self) -> Option<(Application, T)>;
}

impl<'a> DevicesExt<&'a SimpleDescriptor> for (&'a Endpoint, &'a SimpleDescriptor) {
    fn application_eps_with_basic_cluster(self) -> Option<(Application, &'a SimpleDescriptor)> {
        if let Endpoint::Application(application) = self.0
            && self.1.input_clusters().contains(&ClusterId::Basic.into())
        {
            Some((*application, self.1))
        } else {
            None
        }
    }
}

impl<'a> DevicesExt<&'a EndpointInfo> for (&'a Endpoint, &'a EndpointInfo) {
    fn application_eps_with_basic_cluster(self) -> Option<(Application, &'a EndpointInfo)> {
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
