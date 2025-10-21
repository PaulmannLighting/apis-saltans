use bitflags::bitflags;

/// Ballast status attribute.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct BallastStatus(u8);

bitflags! {
    impl BallastStatus: u8 {
        /// Flag, indicating that the ballast is not fully operational.
        const NON_OPERATIONAL = 0b0000_0001;
        /// Flag, indicating that the lamp is not in the socket.
        const LAMP_NOT_IN_SOCKET = 0b0000_0010;
    }
}

impl BallastStatus {
    /// Create a new `BallastStatus` instance.
    #[must_use]
    pub const fn new(status: u8) -> Self {
        Self(status)
    }

    /// Return whether the ballast is not fully operational.
    #[must_use]
    pub const fn is_non_operational(&self) -> bool {
        self.contains(Self::NON_OPERATIONAL)
    }

    /// Return whether the lamp is not in the socket.
    #[must_use]
    pub const fn is_lamp_not_in_socket(&self) -> bool {
        self.contains(Self::LAMP_NOT_IN_SOCKET)
    }

    /// Return whether the ballast is fully operational.
    #[must_use]
    pub const fn is_operational(&self) -> bool {
        !self.is_non_operational()
    }

    /// Return whether the lamp is in the socket.
    #[must_use]
    pub const fn is_lamp_in_socket(&self) -> bool {
        !self.is_lamp_not_in_socket()
    }
}
