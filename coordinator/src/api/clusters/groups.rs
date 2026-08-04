use zb_aps::apsde::{IndividualEndpoint, NetworkDestination};
use zb_core::GroupId;
use zb_core::types::{String, Uint16};
use zb_zcl::groups::{
    AddGroup, AddGroupResponse, GetGroupMembership, GetGroupMembershipResponse, RemoveGroup,
    RemoveGroupResponse,
};

use crate::{Error, StatusExt, Zcl, ZclResponse};

/// Trait for Groups cluster operations.
///
/// Every operation requires the local APS source endpoint.
pub trait Groups {
    /// Lists the group memberships from the device.
    ///
    /// The first await queues the request and returns a [`ZclResponse`]. Await that response to
    /// confirm transmission and receive the membership list.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request cannot be queued. The returned [`ZclResponse`] reports
    /// transmission, reception, and response-conversion errors when awaited.
    fn list(
        &self,
        destination: NetworkDestination,
        source_endpoint: IndividualEndpoint,
    ) -> impl Future<Output = Result<ZclResponse<GetGroupMembershipResponse>, Error>> + Send;

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
    ) -> Result<ZclResponse<GetGroupMembershipResponse>, Error> {
        self.communicate(crate::api::zcl::request(
            destination.into(),
            source_endpoint,
            GetGroupMembership::default(),
        ))
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
            .communicate::<AddGroupResponse>(crate::api::zcl::request(
                destination.into(),
                source_endpoint,
                AddGroup::new(group_id, name.unwrap_or_default()),
            ))
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
            .communicate::<RemoveGroupResponse>(crate::api::zcl::request(
                destination.into(),
                source_endpoint,
                RemoveGroup::new(group_id),
            ))
            .await?
            .await?;
        response
            .status()
            .ensure_success()
            .map(|()| response.group_id())
    }
}
