use bitmap::Bitmap;
use collection::Collection;
use enumeration::Enumeration;
use floating_point::FloatingPoint;
use general::General;
use identifier::Identifier;
use miscellaneous::Miscellaneous;
use ordered_sequence::OrderedSequence;
use signed_integer::SignedInteger;
use string::String;
use time::Time;
use unsigned_integer::UnsignedInteger;

mod array;
mod bitmap;
mod collection;
mod date;
mod enumeration;
mod floating_point;
mod general;
mod identifier;
mod miscellaneous;
mod ordered_sequence;
mod signed_integer;
mod string;
mod structure;
mod time;
mod time_of_day;
mod unsigned_integer;
mod utc_time;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Class {
    Null,
    General(General),
    Logical(bool),
    Bitmap(Bitmap),
    UnsignedInteger(UnsignedInteger),
    SignedInteger(SignedInteger),
    Enumeration(Enumeration),
    FloatingPoint(FloatingPoint),
    String(String),
    OrderedSequence(OrderedSequence),
    Collection(Collection),
    Time(Time),
    Identifier(Identifier),
    Miscellaneous(Miscellaneous),
    Unknown,
}
