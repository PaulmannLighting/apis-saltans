//! Macros for deriving parsing functionality for frame-like data structures.

use parse_zcl_frame::parse_zcl_frame;
use proc_macro::TokenStream;

/// Implement a crate-internal frame parser for the annotated enum.
///
/// # Panics
///
/// This macro will panic if the input type is not an enum with single-field variants.
#[proc_macro_derive(ParseZclFrame)]
pub fn derive_parse_zcl_frame(input: TokenStream) -> TokenStream {
    parse_zcl_frame(input)
}

mod parse_zcl_frame;
