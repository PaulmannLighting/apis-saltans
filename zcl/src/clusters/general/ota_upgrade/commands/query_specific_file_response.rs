use zb_core::{Cluster, Direction};

use crate::macros::zcl_command;
use crate::ota_upgrade::QueryResponse;

zcl_command! {
    /// Reports whether a requested device-specific file is available.
    QuerySpecificFileResponse {
        { Cluster::OtaUpgrade } => OtaUpgrade;
        command_id: 0x09;
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
