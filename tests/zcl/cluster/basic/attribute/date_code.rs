use std::str::FromStr;

use chrono::NaiveDate;
use le_stream::{FromLeStream, ToLeStream};
use zigbee::zcl::basic::{CustomString, DateCode};

#[test]
fn from_str_with_custom() {
    let date_code = DateCode::from_str("20060814Custom").unwrap();
    assert_eq!(
        date_code.date(),
        NaiveDate::from_ymd_opt(2006, 8, 14).unwrap(),
    );
    assert_eq!(date_code.custom(), "Custom");
}

#[test]
fn from_str_without_custom() {
    let date_code = DateCode::from_str("20060814").unwrap();
    assert_eq!(
        date_code.date(),
        NaiveDate::from_ymd_opt(2006, 8, 14).unwrap(),
    );
    assert_eq!(date_code.custom(), "");
}

#[test]
fn to_string() {
    let date_code = DateCode::new(
        NaiveDate::from_ymd_opt(2006, 8, 14).unwrap(),
        "Custom".try_into().unwrap(),
    );
    assert_eq!(date_code.to_string(), "20060814Custom");
}

#[test]
fn from_le_stream() {
    let bytes = "20060814Custom".bytes();
    let date_code = DateCode::from_le_stream(bytes).unwrap();
    assert_eq!(
        date_code.date(),
        NaiveDate::from_ymd_opt(2006, 8, 14).unwrap(),
    );
    assert_eq!(date_code.custom(), "Custom");
}

#[test]
fn to_le_stream() {
    let date_code = DateCode::new(
        NaiveDate::from_ymd_opt(2006, 8, 14).unwrap(),
        CustomString::from_utf8("Custom".bytes().collect()).unwrap(),
    );
    let bytes: Vec<u8> = date_code.to_le_stream().collect();
    assert_eq!(bytes, b"20060814Custom");
}
