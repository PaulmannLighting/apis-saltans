use apis_saltans_core::node::ServerMask;

crate::zdp_command! {
    /// System Server Discovery Request
    derive { Copy }
    SystemServerDiscoveryReq => System_Server_Discovery_req;
    cluster_id: 0x0015;
    group: DeviceAndServiceDiscovery;
    fields {
        server_mask: ServerMask,
    }
    getters {
        /// Returns the server mask of the request.
        #[must_use]
        pub const fn server_mask(self) -> ServerMask {
            self.server_mask
        }
    }
    display {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {{ server_mask: {} }}", Self::NAME, self.server_mask)
        }
    }
    from {
        impl From<ServerMask> for SystemServerDiscoveryReq {
            fn from(server_mask: ServerMask) -> Self {
                Self::new(server_mask)
            }
        }

        impl From<SystemServerDiscoveryReq> for ServerMask {
            fn from(req: SystemServerDiscoveryReq) -> Self {
                req.server_mask()
            }
        }
    }
}
