use tokio::sync::mpsc::{Receiver, Sender};

/// Bridge two channels.
///
/// This function will read all messages from `input`, convert them to the message type of `output`
/// and send it to the output channel.
///
/// This function will terminate once either the input channel is exhausted or the output channel
/// has been closed.
pub async fn bridge<T, U>(mut input: Receiver<T>, output: Sender<U>)
where
    T: Into<U>,
{
    while let Some(msg) = input.recv().await {
        if output.send(msg.into()).await.is_err() {
            break;
        }
    }
}
