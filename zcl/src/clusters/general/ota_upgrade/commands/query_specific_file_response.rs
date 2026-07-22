use zb_core::{Cluster, Direction};

use crate::macros::zcl_command;
use crate::ota_upgrade::QueryResponse;

const COMMAND_ID: u8 = 0x09;

zcl_command! {
    /// Reports whether a requested device-specific file is available.
    QuerySpecificFileResponse {
        { Cluster::OtaUpgrade } => OtaUpgrade;
        command_id: COMMAND_ID;
        direction: Direction::ServerToClient;
        disable_default_response: true;
        fields {
            response: QueryResponse,
        }

        getters {
            /// Return the query result.
            #[must_use]
            pub const fn response(&self) -> QueryResponse {
                self.response
            }
        }
    }
}
