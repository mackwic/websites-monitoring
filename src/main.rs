#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use tokio::sync::mpsc::{self, Receiver, Sender};

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
    ),
    End,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::set_up_logging();
    info!("Starting program");

    let websites = vec![
        "https://scalingo.com",
        "https://google.com",
        "https://free.fr",
        "https://example.com",
        "https://www.clever-cloud.com/en/",
        "https://platform.sh/",
        "https://www.salesforce.com/editions-pricing/sales-cloud/",
        "https://twitter.com",
        "https://facebook.com",
        "https://bing.com",
        "https://imgur.com",
        "https://inexistant.com",
        "http://httpbin.org/status/500",
        "http://httpbin.org/status/404",
        "http://httpbin.org/status/401",
        "http://httpbin.org/status/503",
        "http://httpbin.org/status/504",
    ];
    let websites = websites
        .iter()
        .map(|site| reqwest::Url::parse(site).expect("Url should be valid !"))
        .collect();

    let (sender, receiver) = mpsc::channel(10_000);
    let (sender_end_channel, mut receiver_end_channel) = mpsc::channel(1);

    spawn_receiver(receiver, sender_end_channel);

    async {
        use std::time::Duration;
        use tokio::timer::Interval;
        let mut iteration: u32 = 3;
        let mut interval = Interval::new_interval(Duration::from_millis(5_000));
        let mut sender = sender.clone();

        sender.send(Messages::Start).await.unwrap();
        while iteration > 0 {
            iteration -= 1;
            interval.next().await;
            info!("[iteration:{}] Starting to fetch websites...", iteration);
            fetcher::fetch_all_websites(&websites, sender.clone()).await;
            info!("[iteration:{}] Fetching finished", iteration);
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
