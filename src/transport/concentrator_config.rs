use super::ConcentratorKind;

pub struct ConcentratorConfig {
    kind: ConcentratorKind,
    refresh_minimum: usize,
    refresh_maximum: usize,
    max_hops: usize,
    max_failures: usize,
}
