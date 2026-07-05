//! Bind, unbind, and bind management related ZDP services.

pub use self::bind_req::{Address, AddressMode, BindReq, Destination};
pub use self::bind_rsp::BindRsp;
pub use self::clear_all_bindings_req::ClearAllBindingsReq;
pub use self::unbind_req::UnbindReq;

mod bind_req;
mod bind_rsp;
mod clear_all_bindings_req;
mod unbind_req;

crate::services::zdp_command_group! {
    /// Bind management commands.
    BindManagement {
        BindReq,
        BindRsp,
        UnbindReq,
        ClearAllBindingsReq,
    }
}
