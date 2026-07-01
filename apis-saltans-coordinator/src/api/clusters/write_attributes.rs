use apis_saltans_core::Application;
use apis_saltans_hw::Metadata;
use apis_saltans_zcl::WritableAttribute;
use apis_saltans_zcl::global::write_attributes::{Command, Record, Response};
use macaddr::MacAddr8;

use crate::transceiver::zcl::{Handle, Payload};
use crate::{Coordinator, Error, NetworkManager};

/// Trait to write attributes to a device.
pub trait WriteAttributes {
    /// Write raw attributes to a device.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the communication fails or if the response is not a valid [`Response`].
    fn write_attributes_raw(
        &self,
        ieee_address: MacAddr8,
        endpoint: Application,
        cluster: u16,
        manufacturer_code: Option<u16>,
        records: Box<[Record]>,
    ) -> impl Future<Output = Result<Response, Error>> + Send;

    /// Write attributes to a device.
    ///
    /// # Returns
    ///
    /// Returns a [`Vec`] of [`Result`]s, where each [`Result`] contains the status of the write operation for each attribute.
    ///
    /// - `Ok(id)`: The attribute was successfully written.
    /// - `Err(id)`: The attribute could not be written.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the communication fails or if the response is not a valid [`Response`].
    fn write_attributes<T>(
        &self,
        ieee_address: MacAddr8,
        endpoint: Application,
        attributes: Box<[T]>,
    ) -> impl Future<Output = Result<Vec<Result<u16, u16>>, Error>> + Send
    where
        Self: Sync,
        T: WritableAttribute,
    {
        let records = attributes.into_iter().map(Into::into).collect();

        async move {
            self.write_attributes_raw(ieee_address, endpoint, T::ID, T::MANUFACTURER_CODE, records)
                .await
                .map(|response| response.into_iter().map(TryInto::try_into).collect())
        }
    }
}

impl WriteAttributes for Coordinator {
    async fn write_attributes_raw(
        &self,
        ieee_address: MacAddr8,
        endpoint: Application,
        cluster: u16,
        manufacturer_code: Option<u16>,
        records: Box<[Record]>,
    ) -> Result<Response, Error> {
        #[expect(unsafe_code)]
        // SAFETY: We construct matching metadata from the given cluster ID.
        // Since witing attributes is a global command, we don't need to validate the cluster ID.
        // Hence, the resulting metadata and command are guaranteed to match.
        let payload = unsafe {
            Payload::new(
                Metadata::new(cluster, None, None),
                manufacturer_code,
                Command::new(records),
            )
        };

        self.zcl
            .communicate(
                self.network_manager
                    .get_short_id_from_ieee_address(ieee_address)
                    .await?
                    .ok_or(Error::UnknownDevice(ieee_address))?,
                endpoint,
                payload,
            )
            .await
    }
}
