#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use tokio::sync::mpsc::{self, Receiver, Sender};

mod logging;

#[derive(Debug)]
enum Messages {
    Start,
    StartPingSite(chrono::DateTime<chrono::Local>, reqwest::Url),
    EndPingSite(
        chrono::DateTime<chrono::Local>,
        reqwest::Url,
        Result<(), ()>,
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
        let mut iteration: u32 = 10;
        let mut interval = Interval::new_interval(Duration::from_millis(3_000));
        let mut sender = sender.clone();

        sender.send(Messages::Start).await.unwrap();
        while iteration > 0 {
            iteration -= 1;
            interval.next().await;
            ping_websites(&websites, sender.clone()).await;
        }
        sender.send(Messages::End).await.unwrap();
    }
        .await;

    receiver_end_channel.recv().await;
    debug!("Terminating program");
    Ok(())
}

async fn ping_websites(websites: &Vec<reqwest::Url>, mut sender: Sender<Messages>) {
    for site in websites {
        let start_message = Messages::StartPingSite(chrono::Local::now(), site.clone());
        sender.send(start_message).await.unwrap();
        let res = fetch_site(site).await;

        let end_message = Messages::EndPingSite(chrono::Local::now(), site.clone(), res);
        sender.send(end_message).await.unwrap();
    }
}

async fn fetch_site(website: &reqwest::Url) -> Result<(), ()> {
    debug!("Fetching site: {}", website);

    let res = reqwest::get(website.clone()).await;

    match res {
        Err(e) => {
            error!("Error: {}", e);
            Err(())
        }
        Ok(res) => {
            info!("Success: {:?}", res.status());
            Ok(())
        }
    }
}

fn spawn_receiver(mut receiver: Receiver<Messages>, mut sender_end_channel: Sender<()>) {
    tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            match message {
                Messages::End => break,
                _ => info!("got {:?}", message),
            }
        }
        sender_end_channel
            .send(())
            .await
            .expect("error when sending End acknowledge");
    });
}
