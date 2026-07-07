use core::fmt::{self, Display, Formatter, LowerHex, UpperHex};

use apis_saltans_core::IeeeAddress;

/// Network-layer source information.
///
/// The source always includes the 16-bit network address. The IEEE address is
/// optional because it may not be known for every incoming frame.
#[cfg_attr(
    feature = "le-stream",
    derive(le_stream::FromLeStream, le_stream::ToLeStream)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Source {
    node_id: u16,
    ieee_address: Option<IeeeAddress>,
}

impl Source {
    /// Create source information from a network address and optional IEEE address.
    #[must_use]
    pub const fn new(node_id: u16, ieee_address: Option<IeeeAddress>) -> Self {
        Self {
            node_id,
            ieee_address,
        }
    }

    /// Return the 16-bit network address.
    #[must_use]
    pub const fn node_id(&self) -> u16 {
        self.node_id
    }

    /// Return the IEEE address of the source, if known.
    #[must_use]
    pub const fn ieee_address(&self) -> Option<IeeeAddress> {
        self.ieee_address
    }

    /// Split the source into its network address and optional IEEE address.
    #[must_use]
    pub const fn into_parts(self) -> (u16, Option<IeeeAddress>) {
        (self.node_id, self.ieee_address)
    }

    fn format<T, U>(&self, f: &mut Formatter<'_>, node_id_f: T, ieee_address_f: U) -> fmt::Result
    where
        T: FnOnce(&u16, &mut Formatter<'_>) -> fmt::Result,
        U: FnOnce(&IeeeAddress, &mut Formatter<'_>) -> fmt::Result,
    {
        node_id_f(&self.node_id, f)?;
        f.write_str(" (")?;

        if let Some(ieee_address) = self.ieee_address {
            ieee_address_f(&ieee_address, f)?;
        } else {
            f.write_str("N/A")?;
        }

        f.write_str(")")
    }
}

impl Display for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.format(f, Display::fmt, Display::fmt)
    }
}

impl LowerHex for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.format(f, LowerHex::fmt, LowerHex::fmt)
    }
}

impl UpperHex for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.format(f, UpperHex::fmt, UpperHex::fmt)
    }
}
