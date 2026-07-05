use apis_saltans_core::types::{Bool, OctStr, String, Type, Uint8, Uint16, Uint24};

pub use self::errors::{InvalidType, ParseAttributeError};
use crate::clusters::general::alarms::AlarmCount;
use crate::clusters::general::basic::{
    AlarmMask, DateCode, DisableLocalConfig, GenericDeviceClass, PhysicalEnvironment, PowerSource,
};
use crate::global::write_attributes::Record;

mod errors;

/// A trait to allow the reading of attributes by their respective IDs in a type-safe manner.
pub trait ReadableAttribute: TryFrom<u16, Error = u16> + Into<u16> {
    /// The type of attribute, usually an enum, which is returned from the readable.
    type Attribute: TryFrom<(Self, Type), Error = InvalidType<Self>>;

    /// The manufacturer code of the attribute, if any.
    const MANUFACTURER_CODE: Option<u16> = None;
}

/// A trait to allow the writing of attribute values in a type-safe manner.
pub trait WritableAttribute: Into<Record> {
    /// The manufacturer code of the attribute, if any.
    const MANUFACTURER_CODE: Option<u16> = None;

    /// The ID of the attribute.
    fn id(&self) -> u16;
}

#[doc(hidden)]
#[allow(clippy::result_large_err)]
pub trait TryFromAttributeType: Sized {
    fn try_from_attribute_type(typ: Type) -> Result<Self, Type>;
}

impl TryFromAttributeType for Type {
    fn try_from_attribute_type(typ: Type) -> Result<Self, Type> {
        Ok(typ)
    }
}

macro_rules! impl_try_from_attribute_type {
    ($($ty:ty),* $(,)?) => {
        $(
            impl TryFromAttributeType for $ty {
                fn try_from_attribute_type(typ: Type) -> Result<Self, Type> {
                    typ.try_into()
                }
            }
        )*
    };
}

impl_try_from_attribute_type!(
    AlarmCount,
    AlarmMask,
    Bool,
    DateCode,
    DisableLocalConfig,
    GenericDeviceClass,
    PhysicalEnvironment,
    PowerSource,
    Uint8,
    Uint16,
    Uint24,
);

impl<const CAPACITY: usize> TryFromAttributeType for OctStr<CAPACITY> {
    fn try_from_attribute_type(typ: Type) -> Result<Self, Type> {
        typ.try_into()
    }
}

impl<const CAPACITY: usize> TryFromAttributeType for String<CAPACITY> {
    fn try_from_attribute_type(typ: Type) -> Result<Self, Type> {
        typ.try_into()
    }
}
