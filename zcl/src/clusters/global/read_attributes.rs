//! Reading Attributes Command and Response.

use zb_core::Direction;

pub use self::read_attributes_status::ReadAttributesStatus;
use crate::macros::zcl_command;
use crate::storage::{AttributeIds, Records};
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
            attribute_ids: AttributeIds,
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
            attribute_values: Records<ReadAttributesStatus>,
        }

        getters {
            /// Return every record in wire order, including unknown attribute IDs and failures.
            #[must_use]
            pub fn records(&self) -> &[ReadAttributesStatus] {
                &self.attribute_values
            }
        }

        from_le_stream {
            fn from_le_stream<I>(bytes: I) -> Option<Self>
            where
                I: Iterator<Item = u8>,
            {
                Some(Self { attribute_values: crate::storage::read_records(bytes)? })
            }
        }

        impl {
            impl Response {
                /// Returns an iterator over the parsed attribute values in the response.
                #[cfg_attr(not(feature = "alloc"), allow(clippy::result_large_err, reason = "bounded errors retain rejected values inline"))]
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

            impl<T> From<Response> for Records<Result<T::Attribute, ParseAttributeError<T>>>
            where
                T: Readable,
            {
                fn from(response: Response) -> Self {
                    #[cfg(feature = "alloc")]
                    { response.parse::<T>().collect() }
                    #[cfg(not(feature = "alloc"))]
                    { Self::try_from_iter(response.parse::<T>()).expect("one result per bounded input record") }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use le_stream::FromLeStream;

    use super::Response;
    use crate::Frame;

    const SUCCESS: [u8; 5] = [0, 0, 0, 0x10, 1];
    const FAILURE: [u8; 3] = [0x34, 0x12, 0x86];
    const ON_OFF_CLUSTER: u16 = 0x0006;
    const RESPONSE_HEADER: [u8; 3] = [0x18, 0x42, 0x01];

    #[test]
    fn exposes_typed_records_and_rejects_malformed_tails() {
        let single = Response::from_le_stream(SUCCESS.into_iter()).unwrap();
        assert!(single.records()[0].clone().into_parts().1.is_ok());
        assert!(Response::from_le_stream(SUCCESS.into_iter().chain([0, 0, 0])).is_none());

        let wire = SUCCESS.into_iter().chain(FAILURE);
        #[cfg(not(feature = "alloc"))]
        if crate::storage::RECORDS_CAPACITY == 1 {
            assert!(Response::from_le_stream(wire).is_none());
            return;
        }
        let response = Response::from_le_stream(wire.clone()).unwrap();
        let [success, failure] = response.records() else {
            panic!("expected both records");
        };
        let (id, value) = success.clone().into_parts();
        assert_eq!(id, 0);
        assert!(value.is_ok());
        let (id, error) = failure.clone().into_parts();
        assert_eq!(id, 0x1234);
        assert!(error.is_err());
        assert!(Response::from_le_stream(wire.chain([0, 0, 0])).is_none());
    }

    #[test]
    fn frame_parser_rejects_a_malformed_read_response_tail() {
        let wire = RESPONSE_HEADER.into_iter().chain(SUCCESS);
        assert!(Frame::parse(ON_OFF_CLUSTER, wire.clone()).is_ok());
        assert!(Frame::parse(ON_OFF_CLUSTER, wire.chain([0, 0, 0])).is_err());
    }
}
