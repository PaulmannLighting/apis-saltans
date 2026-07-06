pub use self::cluster_id::ClusterId;
use crate::Profile;

mod cluster_id;

/// Trait to identify Zigbee zcl.
pub trait Cluster<T = u16> {
    /// The cluster identifier.
    const ID: T;

    /// The profile.
    const PROFILE: Profile;

    /// The manufacturer code, if any.
    const MANUFACTURER_CODE: Option<u16> = None;
}

impl<T> Cluster<u16> for T
where
    T: Cluster<ClusterId>,
{
    const ID: u16 = T::ID.as_u16();
    const PROFILE: Profile = T::PROFILE;
    const MANUFACTURER_CODE: Option<u16> = T::MANUFACTURER_CODE;
}
