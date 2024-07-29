/// TODO: the code below will deadlock because it's using std's channels,
///  which are not async-aware.
///  Rewrite it to use `tokio`'s channels primitive (you'll have to touch
///  the testing code too, yes).
///
/// Can you understand the sequence of events that can lead to a deadlock?
use tokio::sync::oneshot;

#[derive(Debug)]
pub struct Message {
    payload: String,
    response_channel: oneshot::Sender<Message>,
}

/// Replies with `pong` to any message it receives, setting up a new
/// channel to continue communicating with the caller.
pub async fn pong(mut receiver: oneshot::Receiver<Message>) {
    loop {
        let (sender, new_receiver) = oneshot::channel();
        if let Ok(msg) = receiver.await {
            println!("Pong received: {}", msg.payload);
            msg.response_channel
                .send(Message {
                    payload: "pong".into(),
                    response_channel: sender,
                })
                .unwrap();
        }
        receiver = new_receiver;
    }
}

#[cfg(test)]
mod tests {
    use crate::{pong, Message};
    use tokio::sync::oneshot;

    #[tokio::test]
    async fn ping() {
        let (sender, receiver) = oneshot::channel();
        let (response_sender, response_receiver) = oneshot::channel();
        sender
            .send(Message {
                payload: "pong".into(),
                response_channel: response_sender,
            })
            .unwrap();

        tokio::spawn(pong(receiver));

        let answer = response_receiver.await.unwrap().payload;
        assert_eq!(answer, "pong");
    }
}
