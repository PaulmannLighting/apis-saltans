//! Reading Attributes Command and Response.

use core::iter::Empty;
use core::ops::Deref;
use std::boxed::Box;
use std::collections::BTreeMap;
use std::collections::btree_map::IntoIter;

use apis_saltans_core::Direction;
use apis_saltans_core::types::Type;

pub use self::read_attributes_status::ReadAttributesStatus;
use crate::macros::zcl_command;
use crate::{ParseAttributeError, ReadableAttribute};

mod read_attributes_status;

zcl_command! {
    /// Read Attributes Command.
    Command {
        Global;
        command_id: 0x00;
        direction: Direction::ClientToServer;
        response: Response;
        => crate::global::ReadAttributes(box);
        fields {
            attribute_ids: Box<[u16]>,
        }

        getters {
            /// Returns the attribute IDs of the command.
            #[must_use]
            pub fn attribute_ids(&self) -> &[u16] {
                &self.attribute_ids
            }
        }
    }
}

zcl_command! {
    /// Read Attributes Response.
    Response {
        Global;
        command_id: 0x01;
        direction: Direction::ServerToClient;
        => crate::global::ReadAttributesResponse(box);
        fields {
            attribute_values: BTreeMap<u16, Type>,
        }

        from_le_stream {
            fn from_le_stream<T>(bytes: T) -> Option<Self>
            where
                T: Iterator<Item = u8>,
            {
                Box::<[ReadAttributesStatus]>::from_le_stream(bytes).map(|items| Self {
                    attribute_values: items
                        .into_iter()
                        .map(ReadAttributesStatus::into_parts)
                        .filter_map(|(attribute_id, result)| {
                            result.ok().map(|value| (attribute_id, value))
                        })
                        .collect(),
                })
            }
        }

        to_le_stream {
            type Iter = Empty<u8>;

            fn to_le_stream(self) -> Self::Iter {
                todo!("Not implemented")
            }
        }

        impl {
            impl Response {
                /// Returns an iterator over the parsed attribute values in the response.
                pub fn parse<T>(
                    self,
                ) -> impl Iterator<Item = Result<T::Attribute, ParseAttributeError<T>>>
                where
                    T: ReadableAttribute,
                {
                    self.attribute_values.into_iter().map(|(id, typ)| {
                        T::try_from(id)
                            .map_err(ParseAttributeError::InvalidId)
                            .and_then(|id| T::Attribute::try_from((id, typ)).map_err(Into::into))
                    })
                }
            }

            impl<T> From<Response> for Box<[Result<T::Attribute, ParseAttributeError<T>>]>
            where
                T: ReadableAttribute,
            {
                fn from(response: Response) -> Self {
                    response.parse::<T>().collect()
                }
            }

            impl Deref for Response {
                type Target = BTreeMap<u16, Type>;

                fn deref(&self) -> &Self::Target {
                    &self.attribute_values
                }
            }

            impl IntoIterator for Response {
                type Item = (u16, Type);
                type IntoIter = IntoIter<u16, Type>;

                fn into_iter(self) -> Self::IntoIter {
                    self.attribute_values.into_iter()
                }
            }
        }
    }
}
