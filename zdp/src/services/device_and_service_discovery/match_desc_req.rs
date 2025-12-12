use le_stream::{FromLeStream, Prefixed, ToLeStream};
use zigbee::Cluster;

use crate::Service;

/// Match Descriptor Request.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct MatchDescReq {
    nwk_addr_of_interest: u16,
    profile_id: u16,
    in_cluster_list: Prefixed<u8, Box<[u16]>>,
    out_cluster_list: Prefixed<u8, Box<[u16]>>,
}

impl MatchDescReq {
    /// Creates a new `MatchDescReq` with the given parameters.
    ///
    /// # Errors
    ///
    /// Returns the cluster list whose size could not be represented as `u8`.
    pub fn new(
        nwk_addr_of_interest: u16,
        profile_id: u16,
        in_cluster_list: &[u16],
        out_cluster_list: &[u16],
    ) -> Result<Self, Box<[u16]>> {
        Ok(Self {
            nwk_addr_of_interest,
            profile_id,
            in_cluster_list: Box::<[u16]>::from(in_cluster_list).try_into()?,
            out_cluster_list: Box::<[u16]>::from(out_cluster_list).try_into()?,
        })
    }

    /// Returns the network address of interest.
    #[must_use]
    pub const fn nwk_addr_of_interest(&self) -> u16 {
        self.nwk_addr_of_interest
    }

    /// Returns the profile ID.
    #[must_use]
    pub const fn profile_id(&self) -> u16 {
        self.profile_id
    }

    /// Returns a reference to the input cluster list.
    #[must_use]
    pub fn in_cluster_list(&self) -> &[u16] {
        &self.in_cluster_list
    }

    /// Returns a reference to the output cluster list.
    #[must_use]
    pub fn out_cluster_list(&self) -> &[u16] {
        &self.out_cluster_list
    }
}

impl Cluster for MatchDescReq {
    const ID: u16 = 0x0006;
}

impl Service for MatchDescReq {
    const NAME: &'static str = "Match_Desc_req";
}
