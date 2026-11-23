use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{Response, Status};
use crate::storage::Records;

// Keep the serialized records field compatible with previous versions.
#[expect(
    clippy::ref_option,
    reason = "serde calls this with a reference to the field"
)]
pub(super) fn serialize<S>(
    records: &Option<Records<Status>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    records.as_deref().unwrap_or_default().serialize(serializer)
}

pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Records<Status>>, D::Error>
where
    D: Deserializer<'de>,
{
    Records::deserialize(deserializer).map(|records| Response::new(records).records)
}
