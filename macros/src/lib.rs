//! Macros for deriving parsing functionality for aps-like data structures.

use proc_macro::TokenStream;

use self::parse_zcl_cluster::parse_zcl_cluster;
use self::parse_zcl_frame::parse_zcl_frame;

mod parse_zcl_cluster;
mod parse_zcl_frame;

/// Implement a crate-internal aps parser for an enum representing ZCL zcl.
///
/// # Panics
///
/// This macro will panic if the input type is not an enum with single-field variants.
#[proc_macro_derive(ParseZclCluster)]
pub fn derive_parse_zcl_cluster(input: TokenStream) -> TokenStream {
    parse_zcl_cluster(input)
}

/// Implement a crate-internal aps parser for an enum representing a ZCL cluster's commands.
///
/// # Panics
///
/// This macro will panic if the input type is not an enum with single-field variants.
#[proc_macro_derive(ParseZclFrame)]
pub fn derive_parse_zcl_frame(input: TokenStream) -> TokenStream {
    parse_zcl_frame(input)
}
