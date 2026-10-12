use zb_aps::apsde::{IndividualEndpoint, NetworkDestination};
use zb_core::GroupId;
use zb_core::types::{String, Uint16};
use zb_zcl::groups::{AddGroup, GetGroupMembership, GetGroupMembershipResponse, RemoveGroup};

use crate::{Error, StatusExt, Zcl};

/// Trait for Groups cluster operations.
///
/// Every operation requires the local APS source endpoint.
pub trait Groups {
    /// Lists the group memberships from the device.
    ///
    /// Awaiting this operation completes transmission and returns the membership list.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if queueing, transmission, reception, or response conversion fails.
    fn list(
        &self,
        destination: NetworkDestination,
        source_endpoint: IndividualEndpoint,
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
        destination: NetworkDestination,
        source_endpoint: IndividualEndpoint,
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
        destination: NetworkDestination,
        source_endpoint: IndividualEndpoint,
        group_id: GroupId,
    ) -> impl Future<Output = Result<Uint16, Error>> + Send;
}

impl<T> Groups for T
where
    T: Zcl + Sync,
{
    async fn list(
        &self,
        destination: NetworkDestination,
        source_endpoint: IndividualEndpoint,
    ) -> Result<GetGroupMembershipResponse, Error> {
        self.communicate_command(
            destination.into(),
            source_endpoint,
            GetGroupMembership::default(),
        )
        .await?
        .await
    }

    async fn add(
        &self,
        destination: NetworkDestination,
        source_endpoint: IndividualEndpoint,
        group_id: GroupId,
        name: Option<String>,
    ) -> Result<Uint16, Error> {
        let response = self
            .communicate_command(
                destination.into(),
                source_endpoint,
                AddGroup::new(group_id, name.unwrap_or_default()),
            )
            .await?
            .await?;

        response
            .status()
            .ensure_success()
            .map(|()| response.group_id())
    }

    async fn remove(
        &self,
        destination: NetworkDestination,
        source_endpoint: IndividualEndpoint,
        group_id: GroupId,
    ) -> Result<Uint16, Error> {
        let response = self
            .communicate_command::<_>(
                destination.into(),
                source_endpoint,
                RemoveGroup::new(group_id),
            )
            .await?
            .await?;
        response
            .status()
            .ensure_success()
            .map(|()| response.group_id())
    }
}
