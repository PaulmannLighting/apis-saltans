use std::fmt::Debug;

use log::error;
use tokio::sync::mpsc::error::SendError;

/// Log errors when sending to actors.
pub fn log_send_error<T>(receiver_name: &str) -> impl FnOnce(SendError<T>)
where
    T: Debug,
{
    move |error| error!("Failed to send {:?} to {receiver_name}: {error}", error.0)
}
