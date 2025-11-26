/// A ZDP client service.
pub trait Service {
    const NAME: &'static str;
    const CLUSTER_ID: u16;
}
