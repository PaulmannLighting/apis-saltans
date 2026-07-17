use zb_core::GroupId;
use zb_core::destination::Device;
use zb_core::types::{String, Uint16};
use zb_zcl::Status;
use zb_zcl::groups::{AddGroup, GetGroupMembership, GetGroupMembershipResponse, RemoveGroup};

use crate::{Error, StatusExt, Zcl};

/// Trait for Groups cluster operations.
pub trait Groups {
    /// Lists the group memberships from the device.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn list(
        &self,
        device: Device,
    ) -> impl Future<Output = Result<GetGroupMembershipResponse, Error>> + Send;

    /// Adds the device to a group.
    ///
    /// If `name` is [`None`], an empty group name is sent.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed or if the device
    /// rejected the group addition.
    fn add(
        &self,
        device: Device,
        group_id: GroupId,
        name: Option<String>,
    ) -> impl Future<Output = Result<Uint16, Error>> + Send;

    /// Removes the device from a group.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed or if the device
    /// rejected the group removal.
    fn remove(
        &self,
        device: Device,
        group_id: GroupId,
    ) -> impl Future<Output = Result<Uint16, Error>> + Send;
}

impl<T> Groups for T
where
    T: Zcl + Sync,
{
    async fn list(&self, device: Device) -> Result<GetGroupMembershipResponse, Error> {
        self.communicate(device, GetGroupMembership::default())
            .await
    }

    async fn add(
        &self,
        device: Device,
        group_id: GroupId,
        name: Option<String>,
    ) -> Result<Uint16, Error> {
        let response = self
            .communicate(device, AddGroup::new(group_id, name.unwrap_or_default()))
            .await?;

        let status = response.status();

        if Ok(Status::Success) == status {
            Ok(response.group_id())
        } else {
            Err(status.into())
        }
    }

    async fn remove(&self, device: Device, group_id: GroupId) -> Result<Uint16, Error> {
        let response = self.communicate(device, RemoveGroup::new(group_id)).await?;
        response
            .status()
            .ensure_success()
            .map(|()| response.group_id())
    }
}
