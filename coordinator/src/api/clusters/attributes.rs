use zb_core::destination::Device;
use zb_zcl::global::configure_reporting;
use zb_zcl::{ParseAttributeError, Readable, Reportable, Writable};

use self::configure_reporting_request::ConfigureReportingRequest;
use self::read_attributes_request::ReadAttributesRequest;
use self::write_attributes_request::WriteAttributesRequest;
use crate::Error;
use crate::api::zcl::Zcl;

mod configure_reporting_request;
mod read_attributes_request;
mod write_attributes_request;

/// Result of reading an attribute.
pub type ReadAttributeResult<T> = Result<<T as Readable>::Attribute, ParseAttributeError<T>>;

/// Result of writing an attribute.
pub type WriteAttributeResult = Result<u16, u16>;

/// Trait for ZCL global attribute operations.
///
/// The `device` argument contains the target short address and endpoint. Applications are
/// responsible for discovering and storing those addresses before using this trait.
pub trait Attributes {
    /// Configure a device to send reports for attributes.
    ///
    /// The attributes supply their own cluster, profile, manufacturer, attribute ID, and type
    /// metadata through the ZCL `Reportable` implementation.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if communication fails or the response is invalid.
    fn configure_reporting<T>(
        &self,
        device: Device,
        attributes: T,
    ) -> impl Future<Output = Result<configure_reporting::Response, Error>> + Send
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
        device: Device,
        attributes: T,
    ) -> impl Future<Output = Result<Box<[ReadAttributeResult<T::Item>]>, Error>> + Send
    where
        Self: Sync,
        T: IntoIterator<Item: Readable> + Send;

    /// Write typed attributes to a device.
    ///
    /// Each result contains the ID of an attribute that was written successfully or failed.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if communication fails or the response is invalid.
    fn write<T>(
        &self,
        device: Device,
        attributes: T,
    ) -> impl Future<Output = Result<Vec<WriteAttributeResult>, Error>> + Send
    where
        Self: Sync,
        T: IntoIterator<Item: Writable> + Send;
}

impl<T> Attributes for T
where
    T: Zcl + Sync,
{
    async fn configure_reporting<U>(
        &self,
        device: Device,
        attributes: U,
    ) -> Result<configure_reporting::Response, Error>
    where
        U: IntoIterator<Item: Reportable>,
    {
        self.communicate(device, ConfigureReportingRequest(attributes))
            .await
    }

    async fn read<U>(
        &self,
        device: Device,
        attributes: U,
    ) -> Result<Box<[ReadAttributeResult<U::Item>]>, Error>
    where
        U: IntoIterator<Item: Readable> + Send,
    {
        Ok(self
            .communicate(device, ReadAttributesRequest(attributes))
            .await?
            .into())
    }

    async fn write<U>(
        &self,
        device: Device,
        attributes: U,
    ) -> Result<Vec<WriteAttributeResult>, Error>
    where
        U: IntoIterator<Item: Writable> + Send,
    {
        Ok(self
            .communicate(device, WriteAttributesRequest(attributes))
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
