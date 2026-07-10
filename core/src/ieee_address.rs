use core::fmt::{self, Display, Formatter, LowerHex, UpperHex};
use core::str::FromStr;

use le_stream::{FromLeStream, ToLeStream};
use macaddr::{MacAddr8, ParseError};

/// Alias for an IEEE EUI-64 address.
///
/// Zigbee specifications often use the term EUI-64 for the 64-bit IEEE address
/// assigned to a device. This alias is provided for APIs and data structures
/// that use that terminology.
pub type Eui64 = IeeeAddress;

/// A Zigbee IEEE address.
///
/// This is a transparent protocol newtype around [`MacAddr8`]. It serializes to
/// and from little-endian byte streams like the wrapped address and, with the
/// `serde` feature enabled, serializes through the address display format.
#[cfg_attr(
    feature = "serde",
    cfg_eval::cfg_eval,
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize)
)]
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, FromLeStream, ToLeStream,
)]
pub struct IeeeAddress(
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::DisplayFromStr"))] MacAddr8,
);

impl IeeeAddress {
    /// Create a new IEEE address from its eight address octets.
    ///
    /// The octets are passed in display order, matching [`MacAddr8::new`].
    #[expect(clippy::too_many_arguments, clippy::many_single_char_names)]
    #[must_use]
    pub const fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8) -> Self {
        Self(MacAddr8::new(a, b, c, d, e, f, g, h))
    }
}

impl Display for IeeeAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<MacAddr8> for IeeeAddress {
    fn from(addr: MacAddr8) -> Self {
        Self(addr)
    }
}

impl From<IeeeAddress> for MacAddr8 {
    fn from(eui64: IeeeAddress) -> Self {
        eui64.0
    }
}

impl FromStr for IeeeAddress {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self)
    }
}

impl LowerHex for IeeeAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl UpperHex for IeeeAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::vec;
    use alloc::vec::Vec;

    use le_stream::{FromLeStream, ToLeStream};

    use super::{Eui64, IeeeAddress};

    const ADDRESS: IeeeAddress = IeeeAddress::new(1, 2, 3, 4, 5, 6, 7, 8);

    #[test]
    fn eui64_alias_uses_ieee_address() {
        let address: Eui64 = ADDRESS;
        assert_eq!(address, ADDRESS);
    }

    #[test]
    fn to_le_stream_serializes_little_endian() {
        let bytes: Vec<_> = ADDRESS.to_le_stream().collect();
        assert_eq!(bytes, vec![8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn from_le_stream_deserializes_little_endian() {
        let bytes = vec![8, 7, 6, 5, 4, 3, 2, 1];
        let address = IeeeAddress::from_le_stream(bytes.into_iter()).expect("valid address");

        assert_eq!(address, ADDRESS);
    }

    #[cfg(feature = "serde")]
    mod serde_tests {
        use serde::de::{Deserializer, IntoDeserializer, Visitor};
        use serde::ser::Impossible;
        use serde::{Deserialize, Serialize};

        use super::alloc::string::{String, ToString};
        use super::{ADDRESS, IeeeAddress};

        type Error = serde::de::value::Error;
        const ADDRESS_STRING: &str = "01:02:03:04:05:06:07:08";

        struct StringSerializer;

        struct IeeeAddressDeserializer<'a>(&'a str);

        impl<'de> Deserializer<'de> for IeeeAddressDeserializer<'_> {
            type Error = Error;

            fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: Visitor<'de>,
            {
                Err(serde::de::Error::custom("expected a newtype struct"))
            }

            fn deserialize_newtype_struct<V>(
                self,
                _name: &'static str,
                visitor: V,
            ) -> Result<V::Value, Self::Error>
            where
                V: Visitor<'de>,
            {
                visitor.visit_newtype_struct(self.0.into_deserializer())
            }

            serde::forward_to_deserialize_any! {
                bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes
                byte_buf option unit unit_struct seq tuple tuple_struct map struct enum identifier
                ignored_any
            }
        }

        impl serde::Serializer for StringSerializer {
            type Ok = String;
            type Error = Error;
            type SerializeSeq = Impossible<String, Error>;
            type SerializeTuple = Impossible<String, Error>;
            type SerializeTupleStruct = Impossible<String, Error>;
            type SerializeTupleVariant = Impossible<String, Error>;
            type SerializeMap = Impossible<String, Error>;
            type SerializeStruct = Impossible<String, Error>;
            type SerializeStructVariant = Impossible<String, Error>;

            fn serialize_bool(self, _value: bool) -> Result<Self::Ok, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_i8(self, _value: i8) -> Result<Self::Ok, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_i16(self, _value: i16) -> Result<Self::Ok, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_i32(self, _value: i32) -> Result<Self::Ok, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_i64(self, _value: i64) -> Result<Self::Ok, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_i128(self, _value: i128) -> Result<Self::Ok, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_u8(self, _value: u8) -> Result<Self::Ok, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_u16(self, _value: u16) -> Result<Self::Ok, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_u32(self, _value: u32) -> Result<Self::Ok, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_u64(self, _value: u64) -> Result<Self::Ok, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_u128(self, _value: u128) -> Result<Self::Ok, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_f32(self, _value: f32) -> Result<Self::Ok, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_f64(self, _value: f64) -> Result<Self::Ok, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_char(self, _value: char) -> Result<Self::Ok, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
                Ok(value.into())
            }

            fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
            where
                T: Serialize + ?Sized,
            {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_unit_variant(
                self,
                _name: &'static str,
                _variant_index: u32,
                _variant: &'static str,
            ) -> Result<Self::Ok, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_newtype_struct<T>(
                self,
                _name: &'static str,
                value: &T,
            ) -> Result<Self::Ok, Self::Error>
            where
                T: Serialize + ?Sized,
            {
                value.serialize(self)
            }

            fn serialize_newtype_variant<T>(
                self,
                _name: &'static str,
                _variant_index: u32,
                _variant: &'static str,
                _value: &T,
            ) -> Result<Self::Ok, Self::Error>
            where
                T: Serialize + ?Sized,
            {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_tuple_struct(
                self,
                _name: &'static str,
                _len: usize,
            ) -> Result<Self::SerializeTupleStruct, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_tuple_variant(
                self,
                _name: &'static str,
                _variant_index: u32,
                _variant: &'static str,
                _len: usize,
            ) -> Result<Self::SerializeTupleVariant, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_struct(
                self,
                _name: &'static str,
                _len: usize,
            ) -> Result<Self::SerializeStruct, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }

            fn serialize_struct_variant(
                self,
                _name: &'static str,
                _variant_index: u32,
                _variant: &'static str,
                _len: usize,
            ) -> Result<Self::SerializeStructVariant, Self::Error> {
                Err(serde::ser::Error::custom("expected a string"))
            }
        }

        #[test]
        fn serialize_uses_display_string() {
            let serialized = ADDRESS.serialize(StringSerializer).expect("valid address");

            assert_eq!(serialized, ADDRESS_STRING);
        }

        #[test]
        fn deserialize_uses_display_string() {
            assert_eq!(ADDRESS.to_string(), ADDRESS_STRING);

            let address = IeeeAddress::deserialize(IeeeAddressDeserializer(ADDRESS_STRING))
                .expect("valid address");

            assert_eq!(address, ADDRESS);
        }
    }
}
