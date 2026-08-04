use zb_aps::apsde::{IndividualEndpoint, NetworkDestination};
use zb_core::{ClusterSpecific, Profiled};
use zb_zcl::global::{configure_reporting, read_attributes, write_attributes};
use zb_zcl::{ParseAttributeError, Readable, Reportable, Writable};

use self::configure_reporting_request::frame as configure_reporting_frame;
use self::read_attributes_request::frame as read_attributes_frame;
use self::write_attributes_request::frame as write_attributes_frame;
use crate::api::zcl::Zcl;
use crate::{Error, ZclResponse};

mod configure_reporting_request;
mod read_attributes_request;
mod write_attributes_request;

/// Result of reading an attribute.
pub type ReadAttributeResult<T> = Result<<T as Readable>::Attribute, ParseAttributeError<T>>;

/// Result of writing an attribute.
pub type WriteAttributeResult = Result<u16, u16>;

/// Trait for ZCL global attribute operations.
///
/// The `destination` argument contains the target NWK address and individual endpoint. Applications are
/// responsible for discovering and storing those addresses before using this trait. Every
/// operation also requires the local APS source endpoint.
pub trait Attributes {
    /// Configure a device to send reports for attributes.
    ///
    /// The attributes supply their own cluster, profile, manufacturer, attribute ID, and type
    /// metadata through the ZCL `Reportable` implementation.
    ///
    /// The first await queues the request and returns a [`ZclResponse`]. Await that response to
    /// confirm transmission and receive the device's configure-reporting response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request cannot be queued. The returned [`ZclResponse`] reports
    /// transmission, reception, and response-conversion errors when awaited.
    fn configure_reporting<T>(
        &self,
        destination: NetworkDestination,
        source_endpoint: IndividualEndpoint,
        attributes: T,
    ) -> impl Future<Output = Result<ZclResponse<configure_reporting::Response>, Error>> + Send
    where
        Self: Sync,
        T: IntoIterator<Item: Reportable + Send, IntoIter: Send> + Send;

    /// Read typed attributes from a device.
    ///
    /// Each returned element is either a parsed attribute value or a ZCL parse/status error for
    /// that attribute.
    ///
    /// # Errors
    ///
    /// Returns an [Error] if communication fails or the response is invalid.
    fn read<T>(
        &self,
        destination: NetworkDestination,
        source_endpoint: IndividualEndpoint,
        attributes: T,
    ) -> impl Future<Output = Result<Box<[ReadAttributeResult<T::Item>]>, Error>> + Send
    where
        Self: Sync,
        T: IntoIterator<Item: Readable, IntoIter: Send> + Send;

    /// Write typed attributes to a device.
    ///
    /// Each result contains the ID of an attribute that was written successfully or failed.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if communication fails or the response is invalid.
    fn write<T>(
        &self,
        destination: NetworkDestination,
        source_endpoint: IndividualEndpoint,
        attributes: T,
    ) -> impl Future<Output = Result<Vec<WriteAttributeResult>, Error>> + Send
    where
        Self: Sync,
        T: IntoIterator<Item: Writable, IntoIter: Send> + Send;
}

impl<T> Attributes for T
where
    T: Zcl + Sync,
{
    async fn configure_reporting<U>(
        &self,
        destination: NetworkDestination,
        source_endpoint: IndividualEndpoint,
        attributes: U,
    ) -> Result<ZclResponse<configure_reporting::Response>, Error>
    where
        U: IntoIterator<Item: Reportable, IntoIter: Send> + Send,
    {
        self.communicate(crate::api::zcl::request_with_ids(
            destination.into(),
            source_endpoint,
            <U::Item as Profiled>::PROFILE.as_u16(),
            <U::Item as ClusterSpecific>::ID,
            configure_reporting_frame(attributes),
        ))
        .await
    }

    async fn read<U>(
        &self,
        destination: NetworkDestination,
        source_endpoint: IndividualEndpoint,
        attributes: U,
    ) -> Result<Box<[ReadAttributeResult<U::Item>]>, Error>
    where
        U: IntoIterator<Item: Readable, IntoIter: Send> + Send,
    {
        Ok(self
            .communicate::<read_attributes::Response>(crate::api::zcl::request_with_ids(
                destination.into(),
                source_endpoint,
                <U::Item as Profiled>::PROFILE.as_u16(),
                <U::Item as ClusterSpecific>::ID,
                read_attributes_frame(attributes),
            ))
            .await?
            .await?
            .into())
    }

    async fn write<U>(
        &self,
        destination: NetworkDestination,
        source_endpoint: IndividualEndpoint,
        attributes: U,
    ) -> Result<Vec<WriteAttributeResult>, Error>
    where
        U: IntoIterator<Item: Writable, IntoIter: Send> + Send,
    {
        Ok(self
            .communicate::<write_attributes::Response>(crate::api::zcl::request_with_ids(
                destination.into(),
                source_endpoint,
                <U::Item as Profiled>::PROFILE.as_u16(),
                <U::Item as ClusterSpecific>::ID,
                write_attributes_frame(attributes),
            ))
            .await?
            .await?
            .into_iter()
            .map(TryInto::try_into)
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use zb_core::types::Uint16;
    use zb_zcl::color_control::SendReport;
    use zb_zcl::{Analog, Reportable};

    fn assert_configure_reporting<T>(_: T)
    where
        T: IntoIterator<Item: Reportable + Send, IntoIter: Send> + Send,
    {
    }

    #[test]
    fn test_color_reporting() {
        let requests = [
            SendReport::CurrentX(Analog::new(0, 0, Uint16::MIN)),
            SendReport::CurrentY(Analog::new(0, 0, Uint16::MAX)),
        ];
        assert_configure_reporting(requests);
    }
}
