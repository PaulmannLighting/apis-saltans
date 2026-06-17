use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Endpoint, Profile};

use self::app_flags::AppFlags;
use crate::ByteSizedVec;

mod app_flags;

/// Type alias for the constituent parts of a simple descriptor.
type Parts = (Endpoint, u16, u16, u8, Box<[u16]>, Box<[u16]>);

/// Simple descriptor.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct SimpleDescriptor {
    endpoint: Endpoint,
    profile_id: u16,
    device_id: u16,
    app_flags: AppFlags,
    input_clusters: ByteSizedVec<u16>,
    output_clusters: ByteSizedVec<u16>,
}

impl SimpleDescriptor {
    /// Creates a new `SimpleDescriptor`.
    #[must_use]
    pub const fn new(
        endpoint: Endpoint,
        profile: Profile,
        device_id: u16,
        nibbles: u8,
        input_clusters: ByteSizedVec<u16>,
        output_clusters: ByteSizedVec<u16>,
    ) -> Self {
        Self {
            endpoint,
            profile_id: profile as u16,
            device_id,
            app_flags: AppFlags::from_bits_retain(nibbles),
            input_clusters,
            output_clusters,
        }
    }

    /// Return the endpoint.
    #[must_use]
    pub const fn endpoint(&self) -> Endpoint {
        self.endpoint
    }

    /// Return the profile ID.
    #[must_use]
    pub const fn profile_id(&self) -> u16 {
        self.profile_id
    }

    /// Return the profile.
    ///
    /// # Errors
    ///
    /// Returns the raw profile ID if it cannot be converted into a profile enum.
    pub fn profile(&self) -> Result<Profile, u16> {
        self.profile_id.try_into()
    }

    /// Return the device ID.
    #[must_use]
    pub const fn device_id(&self) -> u16 {
        self.device_id
    }

    /// Return the application flags.
    #[must_use]
    pub const fn app_flags(&self) -> u8 {
        self.app_flags.bits()
    }

    /// Return the version.
    #[must_use]
    pub fn version(&self) -> u8 {
        self.app_flags.version()
    }

    /// Return the input clusters.
    #[must_use]
    pub fn input_clusters(&self) -> &[u16] {
        &self.input_clusters
    }

    /// Return the output clusters.
    #[must_use]
    pub fn output_clusters(&self) -> &[u16] {
        &self.output_clusters
    }

    /// Return the constituent parts of the descriptor.
    #[must_use]
    pub fn into_parts(self) -> Parts {
        (
            self.endpoint,
            self.profile_id,
            self.device_id,
            self.app_flags.version(),
            self.input_clusters.into_iter().collect(),
            self.output_clusters.into_iter().collect(),
        )
    }
}
