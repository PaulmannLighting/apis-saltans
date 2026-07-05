//! Bind, unbind, and bind management related ZDP services.

pub use self::bind_req::{Address, AddressMode, BindReq, Destination};
pub use self::bind_rsp::BindRsp;
pub use self::clear_all_bindings_req::ClearAllBindingsReq;
pub use self::clear_all_bindings_rsp::ClearAllBindingsRsp;
pub use self::unbind_req::UnbindReq;
pub use self::unbind_rsp::UnbindRsp;

mod bind_req;
mod bind_rsp;
mod clear_all_bindings_req;
mod clear_all_bindings_rsp;
mod unbind_req;
mod unbind_rsp;

crate::zdp_command_group! {
    /// Bind management commands.
    BindManagement {
        BindReq,
        BindRsp,
        UnbindReq,
        UnbindRsp,
        ClearAllBindingsReq,
        ClearAllBindingsRsp,
    }
}
