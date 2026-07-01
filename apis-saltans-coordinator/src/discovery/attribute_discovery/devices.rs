use std::collections::BTreeMap;

use apis_saltans_zdp::SimpleDescriptor;
use apis_saltans_core::node::Descriptor;
use apis_saltans_core::{Address, Application, ClusterId, Endpoint};

use super::endpoint_info::EndpointInfo;

/// Type alias for a map of devices to their endpoints.
pub type Devices = BTreeMap<Address, Device>;

#[derive(Debug)]
pub struct Device {
    pub address: Address,
    pub descriptor: Descriptor,
    pub endpoints: BTreeMap<Endpoint, EndpointInfo>,
}

impl From<super::Device> for Device {
    fn from(value: super::Device) -> Self {
        Self {
            address: value.address,
            descriptor: value.descriptor,
            endpoints: value
                .endpoints
                .into_iter()
                .map(|(endpoint, simple_descriptor)| (endpoint, simple_descriptor.into()))
                .collect(),
        }
    }
}

impl From<Device> for crate::Device {
    fn from(value: Device) -> Self {
        Self {
            address: value.address,
            descriptor: value.descriptor,
            endpoints: value
                .endpoints
                .into_iter()
                .map(|(endpoint, info)| (endpoint, info.into()))
                .collect(),
        }
    }
}

/// Helper trait to filter out application endpoints that have the Basic cluster.
pub trait DevicesExt<T> {
    /// Filter out application endpoints that have the Basic cluster.
    ///
    /// This is intended to be used with the [`Iterator::filter_map`] method.
    fn application_eps_with_basic_cluster(self) -> Option<(Application, T)>;
}

impl DevicesExt<SimpleDescriptor> for (Endpoint, SimpleDescriptor) {
    fn application_eps_with_basic_cluster(self) -> Option<(Application, SimpleDescriptor)> {
        if let Endpoint::Application(application) = self.0
            && self.1.input_clusters().contains(&ClusterId::Basic.into())
        {
            Some((application, self.1))
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
