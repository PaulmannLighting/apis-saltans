use apis_saltans_core::Endpoint;
use smarthomelib::TranslateEndpointId;

use crate::Coordinator;

impl TranslateEndpointId<u8> for Coordinator {
    async fn translate_endpoint_id(&self, endpoint: u8) -> Result<Self::EndpointId, Self::Error> {
        endpoint
            .try_into()
            .map_err(Self::Error::InvalidApplicationEndpoint)
    }
}

impl TranslateEndpointId<Endpoint> for Coordinator {
    async fn translate_endpoint_id(
        &self,
        endpoint: Endpoint,
    ) -> Result<Self::EndpointId, Self::Error> {
        if let Endpoint::Application(application) = endpoint {
            return Ok(application);
        }

        Err(Self::Error::InvalidApplicationEndpoint(endpoint.into()))
    }
}
