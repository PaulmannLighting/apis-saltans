use aps::{Control, Destination};

/// An APS frame that has been received.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceivedApsFrame {
    // FIXME: E.g. EZSP does not provide the full APS frame, so we need to keep the control optional.
    control: Option<Control>,
    destination: Destination,
    cluster_id: u16,
    profile_id: u16,
    source_endpoint: u8,
    aps_counter: u8,
    extended: Option<aps::Extended>,
    payload: Box<[u8]>,
}

impl ReceivedApsFrame {
    /// Creates a new `ReceivedApsFrame`.
    #[expect(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        control: Option<Control>,
        destination: Destination,
        cluster_id: u16,
        profile_id: u16,
        source_endpoint: u8,
        aps_counter: u8,
        extended: Option<aps::Extended>,
        payload: Box<[u8]>,
    ) -> Self {
        Self {
            control,
            destination,
            cluster_id,
            profile_id,
            source_endpoint,
            aps_counter,
            extended,
            payload,
        }
    }
}
