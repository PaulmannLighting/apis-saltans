use le_stream::{FromLeStream, Prefixed, ToLeStream};
use zigbee::{Endpoint, Profile};

use self::nibbles::Nibbles;

mod nibbles;

/// Simple descriptor.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct SimpleDescriptor {
    endpoint: Endpoint,
    profile_id: u16,
    device_id: u16,
    nibbles: Nibbles,
    input_clusters: Prefixed<u8, Box<[u16]>>,
    output_clusters: Prefixed<u8, Box<[u16]>>,
}

impl SimpleDescriptor {
    /// Creates a new `SimpleDescriptor`.
    #[must_use]
    pub const fn new(
        endpoint: Endpoint,
        profile_id: u16,
        device_id: u16,
        nibbles: u8,
        input_clusters: Prefixed<u8, Box<[u16]>>,
        output_clusters: Prefixed<u8, Box<[u16]>>,
    ) -> Self {
        Self {
            endpoint,
            profile_id,
            device_id,
            nibbles: Nibbles::from_bits_retain(nibbles),
            input_clusters,
            output_clusters,
        }
    }

    /// Try to create a new `SimpleDescriptor`.
    ///
    /// # Errors
    ///
    /// Returns an error if the input or output clusters cannot be converted into a prefixed boxed slice.
    pub fn try_new(
        endpoint: Endpoint,
        profile_id: u16,
        device_id: u16,
        nibbles: u8,
        input_clusters: &[u16],
        output_clusters: &[u16],
    ) -> Result<Self, Box<[u16]>> {
        let input_clusters = Box::<[u16]>::from(input_clusters).try_into()?;
        let output_clusters = Box::<[u16]>::from(output_clusters).try_into()?;
        Ok(Self::new(
            endpoint,
            profile_id,
            device_id,
            nibbles,
            input_clusters,
            output_clusters,
        ))
    }

    /// Return the endpoint.
    #[must_use]
    pub const fn endpoint(&self) -> Endpoint {
        self.endpoint
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

    /// Return the version.
    #[must_use]
    pub fn version(&self) -> u8 {
        self.nibbles.version()
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
    pub fn into_parts(self) -> (Endpoint, u16, u16, u8, Box<[u16]>, Box<[u16]>) {
        (
            self.endpoint,
            self.profile_id,
            self.device_id,
            self.nibbles.version(),
            self.input_clusters.into_data(),
            self.output_clusters.into_data(),
        )
    }
}
