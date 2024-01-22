#[derive(Clone, Debug, Eq, PartialEq)]
pub enum State {
    Uninitialized,
    Initializing,
    Online,
    Offline,
    Shutdown,
}

pub trait Listener {
    fn network_state_updated(&mut self, state: State);
}
