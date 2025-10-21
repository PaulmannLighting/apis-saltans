use core::str::Utf8Error;

use chrono::Duration;
use le_stream::derive::{FromLeStream, ToLeStream};
use zb::types::{String, Uint8, Uint16};

/// Scene table entry.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct SceneTable {
    group_id: Uint16,
    scene_id: Uint8,
    scene_name: String<16>,
    transition_time: Uint16,
    extension_field_sets: (),    // TODO: More specific type
    transition_time100ms: Uint8, // TODO: Limit to 0x00..0x09.
}

impl SceneTable {
    /// Creates a new `SceneTable` entry.
    #[must_use]
    pub const fn new(
        group_id: Uint16,
        scene_id: Uint8,
        scene_name: String<16>,
        transition_time: Uint16,
        extension_field_sets: (),
        transition_time100ms: Uint8,
    ) -> Self {
        Self {
            group_id,
            scene_id,
            scene_name,
            transition_time,
            extension_field_sets,
            transition_time100ms,
        }
    }

    /// Returns the group ID.
    #[must_use]
    pub fn group_id(&self) -> Option<u16> {
        self.group_id.into()
    }

    /// Returns the scene ID.
    #[must_use]
    pub fn scene_id(&self) -> Option<u8> {
        self.scene_id.into()
    }

    /// Returns the scene name.
    pub fn scene_name(&self) -> Result<&str, Utf8Error> {
        self.scene_name.try_as_str()
    }

    /// Returns the transition time.
    #[must_use]
    pub fn transition_time(&self) -> Option<Duration> {
        Option::<u16>::from(self.transition_time)
            .map(|seconds| Duration::seconds(i64::from(seconds)))
    }

    /// Returns the extension field sets.
    #[must_use]
    pub const fn extension_field_sets(&self) -> () {
        self.extension_field_sets
    }

    /// Returns the transition time in 100ms units.
    #[must_use]
    pub fn transition_time100ms(&self) -> Option<Duration> {
        Option::<u8>::from(self.transition_time100ms)
            .map(|hundred_ms| Duration::milliseconds(i64::from(hundred_ms) * 100))
    }
}
