//! Reading Attributes Command and Response.

use std::boxed::Box;

use zb_core::Direction;

pub use self::read_attributes_status::ReadAttributesStatus;
use crate::macros::zcl_command;
use crate::{ParseAttributeError, Readable};

mod read_attributes_status;

zcl_command! {
    /// Read Attributes Command.
    Command {
        Global;
        command_id: 0x00;
        direction: Direction::ClientToServer;
        response: Response;
        => crate::global::ReadAttributes;
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
        => crate::global::ReadAttributesResponse;
        fields {
            attribute_values: Box<[ReadAttributesStatus]>,
        }

        impl {
            impl Response {
                /// Returns an iterator over the parsed attribute values in the response.
                pub fn parse<T>(
                    self,
                ) -> impl Iterator<Item = Result<T::Attribute, ParseAttributeError<T>>>
                where
                    T: Readable,
                {
                    self.attribute_values.into_iter().map(|status| {
                        let (id, result) = status.into_parts();

                        match result {
                            Ok(typ) => T::try_from(id)
                                .map_err(ParseAttributeError::InvalidId)
                                .and_then(|id| T::Attribute::try_from((id, typ)).map_err(Into::into)),
                            Err(status) => Err(ParseAttributeError::Unsupported {
                                id, status
                            }),
                        }
                    })
                }
            }

            impl<T> From<Response> for Box<[Result<T::Attribute, ParseAttributeError<T>>]>
            where
                T: Readable,
            {
                fn from(response: Response) -> Self {
                    response.parse::<T>().collect()
                }
            }
        }
    }
}
