use le_stream::ToLeStream;
use zigbee::{ClusterId, Direction};

use crate::command::Scoped;
use crate::{Command, Header, HeaderFactory, Scope};

/// Trait to mark commands as customizable.
pub trait Customizable: Sized {
    /// Return a command wrapper with a manufacturer-specific code set.
    fn with_manufacturer_code(self, manufacturer_code: Option<u16>) -> ManufacturerSpecific<Self> {
        ManufacturerSpecific {
            manufacturer_code,
            payload: self,
        }
    }

    /// Return a command wrapper without any manufacturer-specific code set.
    fn native(self) -> ManufacturerSpecific<Self> {
        self.with_manufacturer_code(None)
    }
}

#[derive(Debug)]
pub struct ManufacturerSpecific<T> {
    manufacturer_code: Option<u16>,
    payload: T,
}

impl<T> ClusterId for ManufacturerSpecific<T>
where
    T: ClusterId,
{
    fn cluster_id(&self) -> u16 {
        self.payload.cluster_id()
    }
}

impl<T> Command for ManufacturerSpecific<T>
where
    T: Command,
{
    const ID: u8 = T::ID;
    const DIRECTION: Direction = T::DIRECTION;
    const DISABLE_DEFAULT_RESPONSE: bool = T::DISABLE_DEFAULT_RESPONSE;
}

impl<T> Scoped for ManufacturerSpecific<T>
where
    T: Scoped,
{
    const SCOPE: Scope = T::SCOPE;
}

#[expect(unsafe_code)]
// SAFETY: We forward the appropriate fields of `<T as Command>` and the manufacturer code to the header.
unsafe impl<T> HeaderFactory for ManufacturerSpecific<T>
where
    T: Command + Scoped,
{
    fn header(&self, seq: u8) -> Header {
        Header::new(
            T::SCOPE,
            T::DIRECTION,
            T::DISABLE_DEFAULT_RESPONSE,
            self.manufacturer_code,
            seq,
            T::ID,
        )
    }
}

impl<T> ToLeStream for ManufacturerSpecific<T>
where
    T: ToLeStream,
{
    type Iter = T::Iter;

    fn to_le_stream(self) -> Self::Iter {
        self.payload.to_le_stream()
    }
}
