//! ZCL proxies.

pub use self::device::Proxy as DeviceProxy;
pub use self::endpoint::Proxy as EndpointProxy;

mod device;
mod endpoint;
