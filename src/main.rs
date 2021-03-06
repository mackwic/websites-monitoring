#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use tokio::sync::mpsc::{self, Receiver, Sender};

#[macro_use]
extern crate diesel;

mod fetcher;
mod infrastructure;
mod iteration_ticker;
mod receiver;
mod schema;

// re-export
pub use infrastructure::configuration::Configuration;

#[derive(Debug)]
pub enum Messages {
    StartIteration,
    StartPingSite(chrono::DateTime<chrono::Local>, reqwest::Url),
    EndPingSite(
        chrono::DateTime<chrono::Local>,
        reqwest::Url,
        Result<(), String>,
        chrono::Duration,
    ),
    EndIteration,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    infrastructure::logging::set_up_logging();
    info!("Starting program");
    let config = infrastructure::configuration::load_configuration()?;
    infrastructure::http_server::spawn_http_server();

    let (sender, receiver): (Sender<Messages>, Receiver<Messages>) = mpsc::channel(10_000);
    let (sender_end_channel, mut receiver_end_channel): (Sender<()>, Receiver<()>) =
        mpsc::channel(1);
    receiver::spawn_receiver(receiver, sender_end_channel);
    iteration_ticker::run_iterations(config, sender).await;

    receiver_end_channel.recv().await;
    debug!("Terminating program");
    Ok(())
}
