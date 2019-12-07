use log::debug;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::Messages;

pub fn spawn_receiver(mut receiver: Receiver<Messages>, mut sender_end_channel: Sender<()>) {
    tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            match message {
                Messages::EndIteration => break,
                _ => debug!("got {:?}", message),
            }
        }
        sender_end_channel
            .send(())
            .await
            .expect("error when sending EndIteration acknowledge");
    });
}
