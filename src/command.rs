use std::process::Command;

pub trait Listener {
    fn command_received(&mut self, command: Command);
}
