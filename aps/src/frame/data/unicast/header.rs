//! Header definitions for an APS data unicast frame.

use le_stream::{FromLeStream, ToLeStream};
use zb_core::Endpoint;
use zb_core::endpoint::Reserved;

use crate::{Control, DeliveryMode, Extended, FrameType};

/// Named construction parameters for an APS data header.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct UnicastHeaderParameters {
    /// Whether APS security is enabled.
    pub security: bool,
    /// Whether an APS acknowledgement is requested.
    pub ack_request: bool,
    /// Destination endpoint.
    pub dst_endpoint: Endpoint,
    /// Raw cluster identifier.
    pub cluster_id: u16,
    /// Raw profile identifier.
    pub profile_id: u16,
    /// Source endpoint.
    pub source_endpoint: Endpoint,
    /// APS frame counter.
    pub counter: u8,
    /// Optional extended header.
    pub extended: Option<Extended>,
}

/// A header for an APS unicast frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash, ToLeStream)]
pub struct Header {
    control: Control,
    dst_endpoint: u8,
    cluster_id: u16,
    profile_id: u16,
    source_endpoint: u8,
    counter: u8,
    extended: Option<Extended>,
}

impl Header {
    /// Create a new header for an APS unicast frame.
    #[must_use]
    pub fn new(parameters: UnicastHeaderParameters) -> Self {
        let UnicastHeaderParameters {
            security,
            ack_request,
            dst_endpoint,
            cluster_id,
            profile_id,
            source_endpoint,
            counter,
            extended,
        } = parameters;
        let mut control = Control::empty();
        control.set_frame_type(FrameType::Data);
        control.set_delivery_mode(DeliveryMode::Unicast);
        control.set_extended_header(extended.is_some());

        if security {
            control.insert(Control::SECURITY);
        }

        if ack_request {
            control.insert(Control::ACK_REQUEST);
        }

        Self {
            control,
            dst_endpoint: dst_endpoint.into(),
            cluster_id,
            profile_id,
            source_endpoint: source_endpoint.into(),
            counter,
            extended,
        }
    }

    /// Return the control flags.
    #[must_use]
    pub const fn control(&self) -> Control {
        self.control
    }

    /// Return the destination endpoint ID.
    ///
    /// # Errors
    ///
    /// Returns [`Reserved`] if the stored endpoint ID is reserved.
    pub fn dst_endpoint(&self) -> Result<Endpoint, Reserved> {
        self.dst_endpoint.try_into()
    }

    /// Return the cluster ID.
    #[must_use]
    pub const fn cluster_id(&self) -> u16 {
        self.cluster_id
    }

    /// Return the profile ID.
    #[must_use]
    pub const fn profile_id(&self) -> u16 {
        self.profile_id
    }

    /// Return the source endpoint ID.
    ///
    /// # Errors
    ///
    /// Returns [`Reserved`] if the stored endpoint ID is reserved.
    pub fn source_endpoint(&self) -> Result<Endpoint, Reserved> {
        self.source_endpoint.try_into()
    }

    /// Return the APS counter.
    #[must_use]
    pub const fn counter(&self) -> u8 {
        self.counter
    }

    /// Return the extended header.
    #[must_use]
    pub const fn extended(&self) -> Option<Extended> {
        self.extended
    }
}

impl FromLeStream for Header {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let control = Control::from_le_stream(&mut bytes)?;
        let dst_endpoint = u8::from_le_stream(&mut bytes)?;
        let cluster_id = u16::from_le_stream(&mut bytes)?;
        let profile_id = u16::from_le_stream(&mut bytes)?;
        let source_endpoint = u8::from_le_stream(&mut bytes)?;
        let counter = u8::from_le_stream(&mut bytes)?;
        let extended = control.deserialize_extended_header(&mut bytes).ok()?;

        Some(Self {
            control,
            dst_endpoint,
            cluster_id,
            profile_id,
            source_endpoint,
            counter,
            extended,
        })
    }
}

#[cfg(test)]
mod tests {
    use le_stream::{FromLeStream, ToLeStream};
    use zb_core::Endpoint;

    use super::{Header, UnicastHeaderParameters};
    use crate::{Control, Extended};

    const CLUSTER_ID: u16 = 0x0006;
    const PROFILE_ID: u16 = 0x0104;
    const COUNTER: u8 = 42;

    #[test]
    fn named_options_preserve_control_flags_and_wire_round_trip() {
        for security in [false, true] {
            for ack_request in [false, true] {
                for extended in [None, Some(Extended::default())] {
                    let header = Header::new(UnicastHeaderParameters {
                        security,
                        ack_request,
                        dst_endpoint: Endpoint::Data,
                        cluster_id: CLUSTER_ID,
                        profile_id: PROFILE_ID,
                        source_endpoint: Endpoint::Data,
                        counter: COUNTER,
                        extended,
                    });
                    assert_eq!(header.control().contains(Control::SECURITY), security);
                    assert_eq!(header.control().contains(Control::ACK_REQUEST), ack_request);
                    assert_eq!(
                        header.control().contains(Control::EXTENDED_HEADER),
                        extended.is_some()
                    );
                    assert_eq!(
                        Header::from_le_stream(header.clone().to_le_stream()),
                        Some(header)
                    );
                }
            }
        }
    }
}
