use std::fmt::Display;

use le_stream::{FromLeStream, ToLeStream};
use apis_saltans_core::Cluster;

use crate::{ByteSizedVec, Service};

/// Match Descriptor Request.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct MatchDescReq {
    nwk_addr_of_interest: u16,
    profile_id: u16,
    in_cluster_list: ByteSizedVec<u16>,
    out_cluster_list: ByteSizedVec<u16>,
}

impl MatchDescReq {
    /// Creates a new `MatchDescReq` with the given parameters.
    #[must_use]
    pub const fn new(
        nwk_addr_of_interest: u16,
        profile_id: u16,
        in_cluster_list: ByteSizedVec<u16>,
        out_cluster_list: ByteSizedVec<u16>,
    ) -> Self {
        Self {
            nwk_addr_of_interest,
            profile_id,
            in_cluster_list,
            out_cluster_list,
        }
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

impl Display for MatchDescReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {{ nwk_addr_of_interest: {:#06X}, profile_id: {:#06X}, in_cluster_list: [",
            Self::NAME,
            self.nwk_addr_of_interest,
            self.profile_id,
        )?;
        let mut in_clusters = self.in_cluster_list.iter();

        if let Some(cluster) = in_clusters.next() {
            write!(f, "{cluster:#06X}")?;

            for cluster in in_clusters {
                write!(f, ", {cluster:#06X}")?;
            }
        }

        write!(f, "], out_cluster_list: [")?;

        let mut out_clusters = self.out_cluster_list.iter();

        if let Some(cluster) = out_clusters.next() {
            write!(f, "{cluster:#06X}")?;

            for cluster in out_clusters {
                write!(f, ", {cluster:#06X}")?;
            }
        }

        write!(f, "] }}")
    }
}
