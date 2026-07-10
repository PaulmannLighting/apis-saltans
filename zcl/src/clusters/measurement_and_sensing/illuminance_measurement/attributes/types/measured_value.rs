use core::ops::RangeInclusive;

use apis_saltans_core::types::Uint16;

use crate::macros::zcl_attribute_newtype;

const BASE: f32 = 10.0;
const FACTOR: f32 = 10_000.0;
const OFFSET: u16 = 1;
const VALID_LUX_VALUES: RangeInclusive<u32> = 1..=3_576_000;

zcl_attribute_newtype! {
    /// Illuminance in Lux (lx).
    pub ranged struct Lux(u32) = 1..=3_576_000;
}

zcl_attribute_newtype! {
    /// Measured value of a sensor.
    pub struct MeasuredValue(Uint16) => Uint16;
}

impl MeasuredValue {
    /// Create a new measured value.
    #[must_use]
    pub fn try_new(measured_value: u16) -> Option<Self> {
        Uint16::try_from(measured_value).map(Self::new).ok()
    }

    /// Create a new measured value from a Lux (lx) value.
    #[must_use]
    pub fn try_from_lux(lux: Lux) -> Option<Self> {
        lux.try_into().ok()
    }

    /// Return the raw value.
    #[must_use]
    pub fn raw_value(self) -> Option<u16> {
        self.into_inner().into()
    }

    /// Return the measured value in Lux (lx).
    #[must_use]
    pub fn lux(self) -> Option<Lux> {
        self.try_into().ok()
    }
}

impl TryFrom<Lux> for MeasuredValue {
    type Error = Lux;

    fn try_from(value: Lux) -> Result<Self, Self::Error> {
        lux_to_measured_value(value.into_inner())
            .map(Uint16::new)
            .map(Self::new)
            .ok_or(value)
    }
}

impl TryFrom<MeasuredValue> for Lux {
    type Error = MeasuredValue;

    fn try_from(value: MeasuredValue) -> Result<Self, Self::Error> {
        value
            .raw_value()
            .and_then(measured_value_to_lux)
            .and_then(Self::try_new)
            .ok_or(value)
    }
}

#[expect(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss
)]
fn lux_to_measured_value(lux: u32) -> Option<u16> {
    if VALID_LUX_VALUES.contains(&lux) {
        Some((FACTOR * (lux as f32).log(BASE)).round() as u16 + OFFSET)
    } else {
        None
    }
}

#[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn measured_value_to_lux(measured_value: u16) -> Option<u32> {
    measured_value
        .checked_sub(OFFSET)
        .map(|n| BASE.powf(f32::from(n) / FACTOR).round() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lux_to_measured_value() {
        assert_eq!(
            MeasuredValue::try_from(Lux::try_new(1).unwrap())
                .unwrap()
                .raw_value(),
            Some(1)
        );
        assert_eq!(lux_to_measured_value(5), Some(6991));
        assert_eq!(lux_to_measured_value(10), Some(10001));
        assert_eq!(lux_to_measured_value(20), Some(13011));
        assert_eq!(lux_to_measured_value(100), Some(20001));
        assert_eq!(lux_to_measured_value(500), Some(26991));
        assert_eq!(lux_to_measured_value(1000), Some(30001));
        assert_eq!(lux_to_measured_value(1200), Some(30793));
        assert_eq!(lux_to_measured_value(1400), Some(31462));
        assert_eq!(lux_to_measured_value(3_576_000), Some(65535));
    }

    #[test]
    fn test_measured_value_to_lux() {
        assert_eq!(measured_value_to_lux(1), Some(1));
        assert_eq!(measured_value_to_lux(8_000), Some(6));
        assert_eq!(measured_value_to_lux(65534), Some(3_575_197));
        assert_eq!(measured_value_to_lux(65535), Some(3_576_021));
    }

    #[test]
    fn test_measured_value_to_lux_type() {
        assert_eq!(
            Lux::try_from(MeasuredValue::try_new(1).unwrap())
                .unwrap()
                .into_inner(),
            1
        );
    }
}
