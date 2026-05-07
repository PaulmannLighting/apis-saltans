use zigbee::Cluster;

use crate::Error;

/// Trait for managing ZCL attributes.
pub trait Attributes {
    /// Read attributes from the specified cluster.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn read_raw(
        &self,
        cluster_id: u16,
        attribute_ids: Box<[u16]>,
        manufacturer_code: Option<u16>,
    ) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Read attributes of a specific cluster.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn read<T>(
        &self,
        attributes: &[T],
        manufacturer_code: Option<u16>,
    ) -> impl Future<Output = Result<u8, Error>> + Send
    where
        T: Cluster + Copy + Into<u16>,
    {
        self.read_raw(
            T::ID,
            attributes.iter().copied().map(Into::into).collect(),
            manufacturer_code,
        )
    }
}
