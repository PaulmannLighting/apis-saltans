use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Security status values reported by `APSDE-DATA.indication`.
#[derive(
    Clone, Copy, Debug, Eq, Hash, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive,
)]
#[num_enum(error_type(name = u8, constructor = core::convert::identity))]
#[repr(u8)]
pub enum SecurityStatus {
    /// The ASDU was secured using an APS link key.
    SecuredLinkKey = 0xab,

    /// The ASDU was secured using the NWK key.
    SecuredNetworkKey = 0xac,

    /// The ASDU was received without security.
    Unsecured = 0xaf,
}

/// Security metadata reported by `APSDE-DATA.indication`.
///
/// The key index and device-key-pair entry are present only for link-key
/// security, matching the validity rules of the service primitive.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Security<K = ()> {
    /// The ASDU was received without security.
    Unsecured,

    /// The ASDU was secured using the NWK key.
    NetworkKey,

    /// The ASDU was secured using an APS link key.
    LinkKey {
        /// Index of the key in the APS device-key-pair set.
        key_index: u8,
        /// Implementation-defined handle to the device-key-pair entry.
        device_key_pair_entry: K,
    },
}

impl<K> Security<K> {
    /// Return the APSDE security-status value.
    #[must_use]
    pub const fn status(&self) -> SecurityStatus {
        match self {
            Self::Unsecured => SecurityStatus::Unsecured,
            Self::NetworkKey => SecurityStatus::SecuredNetworkKey,
            Self::LinkKey { .. } => SecurityStatus::SecuredLinkKey,
        }
    }

    /// Transform the device-key-pair handle while preserving the security mode.
    #[must_use]
    pub fn map_key_pair<U, F>(self, map: F) -> Security<U>
    where
        F: FnOnce(K) -> U,
    {
        match self {
            Self::Unsecured => Security::Unsecured,
            Self::NetworkKey => Security::NetworkKey,
            Self::LinkKey {
                key_index,
                device_key_pair_entry,
            } => Security::LinkKey {
                key_index,
                device_key_pair_entry: map(device_key_pair_entry),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Security, SecurityStatus};

    const DEVICE_KEY_PAIR: &str = "device-key-pair";
    const KEY_INDEX: u8 = 3;

    #[test]
    fn link_key_groups_all_conditionally_valid_fields() {
        let security = Security::LinkKey {
            key_index: KEY_INDEX,
            device_key_pair_entry: DEVICE_KEY_PAIR,
        };

        assert_eq!(security.status(), SecurityStatus::SecuredLinkKey);
        assert_eq!(
            security.map_key_pair(str::len),
            Security::LinkKey {
                key_index: KEY_INDEX,
                device_key_pair_entry: DEVICE_KEY_PAIR.len(),
            }
        );
    }
}
