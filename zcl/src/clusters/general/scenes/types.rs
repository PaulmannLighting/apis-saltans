use zb_core::types::Uint8;

use crate::storage::Extensions;

/// A list of scene IDs.
pub type SceneList = heapless::Vec<Uint8, { Uint8::MAX.into_inner() as usize }, u8>;

/// Scene extension field sets.
pub type ExtensionFieldSets = Extensions;
