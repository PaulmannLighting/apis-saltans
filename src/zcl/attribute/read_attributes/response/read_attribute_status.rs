use payload::Payload;

mod payload;

pub struct ReadAttributeStatus {
    id: u16,
    status: u8,
    payload: Option<Payload>,
}
