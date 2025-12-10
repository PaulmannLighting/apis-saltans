//! Attribute macro to implement a discriminant method for enums with a specific representation type.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

pub fn parse_zcl_frame(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let Data::Enum(enumeration) = input.data else {
        unimplemented!("`ParseZclFrame` can only be derived for enums")
    };

    let enum_name = &input.ident;
    let mut match_arms = proc_macro2::TokenStream::new();

    for variant in enumeration.variants {
        assert_eq!(
            variant.fields.len(),
            1,
            "`ParseZclFrame` can only be used with enums with single-field variants"
        );
        let field = variant.fields.into_iter().next().expect(
            "`ParseZclFrame` can only be derived for enums with exactly one field per variant",
        );
        assert!(
            field.ident.is_none(),
            "`ParseZclFrame` can only be used with anonymous fields"
        );

        let variant_name = variant.ident;
        let inner_type = field.ty;

        match_arms.extend(quote! {
            (
                <#inner_type as ::zigbee::Command>::ID,
                <#inner_type as ::zigbee::Command>::DIRECTION
            ) => <#inner_type as ::le_stream::FromLeStream>::from_le_stream(bytes)
                .map(Self::#variant_name)
                .ok_or(crate::ParseFrameError::InsufficientPayload),
        });
    }

    quote! {
        impl #enum_name {
            /// Parses a ZCL frame from a little-endian byte stream given the command ID and direction.
            ///
            /// # Errors
            ///
            /// This function will return [`ParseFrameError`](crate::ParseFrameError) if the command
            /// ID does not correspond to any variant or if the payload could not be parsed.
            pub(crate) fn parse_zcl_frame<T>(
                header: crate::Header,
                bytes: T,
            ) -> ::core::result::Result<Self, crate::ParseFrameError>
            where
                T: ::core::iter::Iterator<Item = u8>,
            {
                match (header.command_id(), header.control().direction()) {
                    #match_arms
                    (command_id, _) => Err(crate::ParseFrameError::InvalidCommandId(command_id)),
                }
            }
        }
    }
    .into()
}
