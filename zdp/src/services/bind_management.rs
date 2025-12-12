//! Bind, unbind and bind management related ZDP services.

pub use self::bind_req::{Address, AddressMode, BindReq, Destination};
pub use self::clear_all_bindings_req::ClearAllBindingsReq;
pub use self::unbind_req::UnbindReq;

mod bind_req;
mod clear_all_bindings_req;
mod unbind_req;

/// Bind management commands.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum BindManagement {
    /// Bind Request
    BindReq(BindReq),
    /// Unbind Request
    UnbindReq(UnbindReq),
    /// Clear All Bindings Request
    ClearAllBindingsReq(ClearAllBindingsReq),
}
