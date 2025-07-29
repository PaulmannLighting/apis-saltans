use le_stream::derive::{FromLeStream, ToLeStream};

/// `FALSE` value, see 2.6.2.5.
const FALSE: u8 = 0x00;
/// `TRUE` value, see 2.6.2.5.
const TRUE: u8 = 0x01;
/// `non-value`, see 2.6.2.5.
const NON_VALUE: u8 = 0xff;

/// A boolean type, represented as a single byte.
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

impl From<u8> for Bool {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<Bool> for u8 {
    fn from(value: Bool) -> Self {
        value.0
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
    fn from_bool() {
        let bool_true: Bool = true.into();
        assert_eq!(bool_true, Bool::TRUE);

        let bool_false: Bool = false.into();
        assert_eq!(bool_false, Bool::FALSE);
    }

    #[test]
    fn from_u8() {
        let bool_from_u8_true: Bool = TRUE.into();
        assert_eq!(bool_from_u8_true, Bool::TRUE);

        let bool_from_u8_false: Bool = FALSE.into();
        assert_eq!(bool_from_u8_false, Bool::FALSE);

        let bool_from_u8_non_value: Bool = NON_VALUE.into();
        assert_eq!(bool_from_u8_non_value, Bool::NON_VALUE);
    }

    #[test]
    fn try_into_bool() {
        let result_true: Result<bool, u8> = Bool::TRUE.try_into();
        assert_eq!(result_true, Ok(true));

        let result_false: Result<bool, u8> = Bool::FALSE.try_into();
        assert_eq!(result_false, Ok(false));

        let result_non_value: Result<bool, u8> = Bool::NON_VALUE.try_into();
        assert_eq!(result_non_value, Err(NON_VALUE));
    }

    #[test]
    fn try_into_option_bool() {
        let result_some_true: Result<Option<bool>, u8> = Bool::TRUE.try_into();
        assert_eq!(result_some_true, Ok(Some(true)));

        let result_some_false: Result<Option<bool>, u8> = Bool::FALSE.try_into();
        assert_eq!(result_some_false, Ok(Some(false)));

        let result_none: Result<Option<bool>, u8> = Bool::NON_VALUE.try_into();
        assert_eq!(result_none, Ok(None));

        let result_invalid: Result<Option<bool>, u8> = Bool(0x02).try_into();
        assert_eq!(result_invalid, Err(0x02));
    }
}
