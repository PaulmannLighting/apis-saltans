use le_stream::{FromLeStream, ToLeStream};

/// `FALSE` value, see 2.6.2.5.
const FALSE: u8 = 0x00;
/// `TRUE` value, see 2.6.2.5.
const TRUE: u8 = 0x01;
/// `non-value`, see 2.6.2.5.
const NON_VALUE: u8 = 0xff;

/// A boolean type, represented as a single byte.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Bool(u8);

impl Bool {
    /// The `true` value.
    pub const TRUE: Self = Self(TRUE);
    /// The `false` value.
    pub const FALSE: Self = Self(FALSE);
    /// The `non-value` (or `null`) value.
    pub const NON_VALUE: Self = Self(NON_VALUE);
}

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        Self(if value { TRUE } else { FALSE })
    }
}

impl TryInto<bool> for Bool {
    type Error = u8;

    fn try_into(self) -> Result<bool, Self::Error> {
        match self.0 {
            TRUE => Ok(true),
            FALSE => Ok(false),
            other => Err(other),
        }
    }
}

impl TryInto<Option<bool>> for Bool {
    type Error = u8;

    fn try_into(self) -> Result<Option<bool>, Self::Error> {
        match self.0 {
            TRUE => Ok(Some(true)),
            FALSE => Ok(Some(false)),
            NON_VALUE => Ok(None),
            other => Err(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_bool_true() {
        let bool_true: Bool = true.into();
        assert_eq!(bool_true, Bool::TRUE);
    }

    #[test]
    fn from_bool_false() {
        let bool_false: Bool = false.into();
        assert_eq!(bool_false, Bool::FALSE);
    }

    #[test]
    fn try_into_bool_true() {
        let result_true: Result<bool, u8> = Bool::TRUE.try_into();
        assert_eq!(result_true, Ok(true));
    }

    #[test]
    fn try_into_bool_false() {
        let result_false: Result<bool, u8> = Bool::FALSE.try_into();
        assert_eq!(result_false, Ok(false));
    }

    #[test]
    fn try_into_bool_non_value() {
        let result_non_value: Result<bool, u8> = Bool::NON_VALUE.try_into();
        assert_eq!(result_non_value, Err(NON_VALUE));
    }

    #[test]
    fn try_into_bool_invalid() {
        let result_non_value: Result<bool, u8> = Bool(32).try_into();
        assert_eq!(result_non_value, Err(32));
    }

    #[test]
    fn try_into_option_bool_true() {
        let result_some_true: Result<Option<bool>, u8> = Bool::TRUE.try_into();
        assert_eq!(result_some_true, Ok(Some(true)));
    }

    #[test]
    fn try_into_option_bool_false() {
        let result_some_false: Result<Option<bool>, u8> = Bool::FALSE.try_into();
        assert_eq!(result_some_false, Ok(Some(false)));
    }

    #[test]
    fn try_into_option_bool_none() {
        let result_none: Result<Option<bool>, u8> = Bool::NON_VALUE.try_into();
        assert_eq!(result_none, Ok(None));
    }

    #[test]
    fn try_into_option_bool_invalid() {
        let result_invalid: Result<Option<bool>, u8> = Bool(0x02).try_into();
        assert_eq!(result_invalid, Err(0x02));
    }
}
