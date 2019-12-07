#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use tokio::sync::mpsc::{self, Receiver, Sender};

mod configuration;
mod fetcher;
mod logging;

#[derive(Debug)]
pub enum Messages {
    Start,
    StartPingSite(chrono::DateTime<chrono::Local>, reqwest::Url),
    EndPingSite(
        chrono::DateTime<chrono::Local>,
        reqwest::Url,
        Result<(), String>,
        chrono::Duration,
    ),
    End,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::set_up_logging();
    info!("Starting program");
    let config = configuration::load_configuration()?;
    let websites = config
        .websites
        .iter()
        .map(|site| reqwest::Url::parse(site).expect("Url should be valid !"))
        .collect();

    let (sender, receiver) = mpsc::channel(10_000);
    let (sender_end_channel, mut receiver_end_channel) = mpsc::channel(1);

    spawn_receiver(receiver, sender_end_channel);
    spawn_http_server();

    async {
        use std::time::Duration;
        use tokio::timer::Interval;
        let mut iteration = 0;
        let mut interval =
            Interval::new_interval(Duration::from_millis(config.interval_between_fetch));
        let mut sender = sender.clone();

        sender.send(Messages::Start).await.unwrap();
        while config.run_forever || iteration <= config.stop_after_iteration {
            info!("[iteration:{}] Starting to fetch websites...", iteration);
            fetcher::fetch_all_websites(&config, &websites, sender.clone()).await;
            info!("[iteration:{}] Fetching finished", iteration);
            iteration += 1;
            interval.next().await;
        }
        sender.send(Messages::End).await.unwrap();
    }
        .await;

    receiver_end_channel.recv().await;
    debug!("Terminating program");
    Ok(())
}

fn spawn_receiver(mut receiver: Receiver<Messages>, mut sender_end_channel: Sender<()>) {
    tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            match message {
                Messages::End => break,
                _ => debug!("got {:?}", message),
            }
        }
        sender_end_channel
            .send(())
            .await
            .expect("error when sending End acknowledge");
    });
}

fn spawn_http_server() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = "127.0.0.1";

    tokio::spawn(async move {
        let server = simple_server::Server::new(|_request, mut response| {
            Ok(response.body(b"Hello Rust!".to_vec()).unwrap())
        });
        server.listen(address, &port);
    });
}
