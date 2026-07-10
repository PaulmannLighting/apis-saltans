use apis_saltans_core::endpoint::Reserved;
use apis_saltans_core::{ByteSizedVec, Endpoint, Profile};
use le_stream::{FromLeStream, ToLeStream};

pub use self::app_flags::AppFlags;

mod app_flags;

/// Length-prefixed ZDP cluster list.
///
/// The simple descriptor wire format stores input and output cluster IDs as
/// byte-sized lists.
pub type Clusters = ByteSizedVec<u16>;

/// Type alias for the constituent parts of a simple descriptor.
type Parts = (u8, u16, u16, u8, Box<[u16]>, Box<[u16]>);

/// ZDP Simple Descriptor payload.
///
/// A simple descriptor describes one endpoint on a Zigbee node: the endpoint ID,
/// application profile, device ID, application version, and the input/output
/// clusters supported by that endpoint.
///
/// The raw endpoint and profile IDs are preserved so descriptors with reserved
/// endpoint values or unknown profiles can still be transported losslessly.
/// Use [`Self::endpoint`] and [`Self::profile`] when callers need the validated
/// domain enums.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct SimpleDescriptor {
    endpoint: u8,
    profile_id: u16,
    device_id: u16,
    app_flags: AppFlags,
    input_clusters: Clusters,
    output_clusters: Clusters,
}

impl SimpleDescriptor {
    /// Create a simple descriptor from validated endpoint and profile values.
    #[must_use]
    pub const fn new(
        endpoint: Endpoint,
        profile: Profile,
        device_id: u16,
        app_flags: AppFlags,
        input_clusters: Clusters,
        output_clusters: Clusters,
    ) -> Self {
        Self {
            endpoint: endpoint.as_u8(),
            profile_id: profile as u16,
            device_id,
            app_flags,
            input_clusters,
            output_clusters,
        }
    }

    /// Return the raw endpoint ID from the descriptor.
    ///
    /// This accessor is lossless and can return values from the reserved
    /// endpoint range. Use [`Self::endpoint`] to convert the value into an
    /// [`Endpoint`] and report reserved values explicitly.
    #[must_use]
    pub const fn endpoint_id(&self) -> u8 {
        self.endpoint
    }

    /// Return the validated endpoint.
    ///
    /// # Errors
    ///
    /// Returns [`Reserved`] if the raw endpoint value is reserved.
    pub fn endpoint(&self) -> Result<Endpoint, Reserved> {
        self.endpoint.try_into()
    }

    /// Return the raw profile ID from the descriptor.
    #[must_use]
    pub const fn profile_id(&self) -> u16 {
        self.profile_id
    }

    /// Return the validated profile.
    ///
    /// # Errors
    ///
    /// Returns the raw profile ID if it cannot be converted into a profile enum.
    pub fn profile(&self) -> Result<Profile, u16> {
        self.profile_id.try_into()
    }

    /// Return the application device ID.
    #[must_use]
    pub const fn device_id(&self) -> u16 {
        self.device_id
    }

    /// Return the raw application flags byte.
    ///
    /// The version is stored in the high nibble. Use [`Self::version`] to
    /// extract it directly.
    #[must_use]
    pub const fn app_flags(&self) -> u8 {
        self.app_flags.bits()
    }

    /// Return the application version from the descriptor flags.
    #[must_use]
    pub fn version(&self) -> u8 {
        self.app_flags.version()
    }

    /// Return the server cluster IDs implemented by the endpoint.
    #[must_use]
    pub fn input_clusters(&self) -> &[u16] {
        &self.input_clusters
    }

    /// Return the client cluster IDs used by the endpoint.
    #[must_use]
    pub fn output_clusters(&self) -> &[u16] {
        &self.output_clusters
    }

    /// Return the constituent raw parts of the descriptor.
    ///
    /// The tuple contains endpoint ID, profile ID, device ID, application
    /// version, input clusters, and output clusters.
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
