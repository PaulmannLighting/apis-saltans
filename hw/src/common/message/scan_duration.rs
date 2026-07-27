use std::fmt::{Display, Formatter};

const MIN_SCAN_DURATION: u8 = 0;
const MAX_SCAN_DURATION: u8 = 14;

/// Zigbee scan-duration exponent.
///
/// Values zero through fourteen use the scan timing defined by the Zigbee
/// specification. The reserved value fifteen is rejected.
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(try_from = "u8", into = "u8")
)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScanDuration(u8);

impl ScanDuration {
    /// Shortest permitted scan-duration exponent.
    pub const MIN: Self = Self(MIN_SCAN_DURATION);

    /// Longest permitted scan-duration exponent.
    pub const MAX: Self = Self(MAX_SCAN_DURATION);

    /// Create a scan duration when the exponent is valid.
    #[must_use]
    pub const fn new(duration: u8) -> Option<Self> {
        if duration <= MAX_SCAN_DURATION {
            Some(Self(duration))
        } else {
            None
        }
    }

    /// Return the scan-duration exponent.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self.0
    }
}

impl Display for ScanDuration {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl From<ScanDuration> for u8 {
    fn from(duration: ScanDuration) -> Self {
        duration.0
    }
}

impl TryFrom<u8> for ScanDuration {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX_SCAN_DURATION, ScanDuration};

    const RESERVED_SCAN_DURATION: u8 = MAX_SCAN_DURATION + 1;

    #[test]
    fn rejects_reserved_scan_duration() {
        assert_eq!(
            ScanDuration::new(MAX_SCAN_DURATION),
            Some(ScanDuration::MAX)
        );
        assert_eq!(ScanDuration::new(RESERVED_SCAN_DURATION), None);
    }
}
