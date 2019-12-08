use log::info;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::timer::Interval;

use crate::fetcher;
use crate::{Configuration, Messages};

pub async fn run_iterations(config: Configuration, sender: Sender<Messages>) {
    let mut iteration: usize = 0;
    let mut interval = Interval::new_interval(Duration::from_millis(config.interval_between_fetch));
    let mut sender = sender.clone();

    let websites = config
        .websites
        .iter()
        .map(|site| reqwest::Url::parse(site).expect("Url should be valid !"))
        .collect();

    sender.send(Messages::StartIteration).await.unwrap();
    while config.run_forever || iteration <= config.stop_after_iteration {
        info!("[iteration:{}] Starting to fetch websites...", iteration);
        fetcher::fetch_all_websites(&config, &websites, sender.clone()).await;
        info!("[iteration:{}] Fetching finished", iteration);
        iteration += 1;
        interval.next().await;
    }
    sender.send(Messages::EndIteration).await.unwrap();
}
