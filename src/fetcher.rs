use futures::future::join_all;
use tokio::sync::mpsc::Sender;

use crate::Messages;

#[allow(clippy::ptr_arg)]
pub async fn fetch_all_websites(websites: &Vec<reqwest::Url>, sender: Sender<Messages>) {
    const HOW_MANY_SITES_IN_PARALLEL: usize = 10;
    for sites_to_fetch_in_parrallel in websites.chunks(HOW_MANY_SITES_IN_PARALLEL) {
        use std::pin::Pin;

        let wait_for_fetch: Vec<Pin<Box<_>>> = sites_to_fetch_in_parrallel
            .iter()
            .map(|site| (site, sender.clone()))
            .map(move |(site, mut sender)| {
                let future = async move {
                    let start_message = Messages::StartPingSite(chrono::Local::now(), site.clone());
                    sender.send(start_message).await.unwrap();
                    let res = fetch_one_site(site).await;

                    let end_message =
                        Messages::EndPingSite(chrono::Local::now(), site.clone(), res);
                    sender.send(end_message).await.unwrap();
                };
                Box::pin(future)
            })
            .collect();
        join_all(wait_for_fetch).await;
    }
}

async fn fetch_one_site(website: &reqwest::Url) -> Result<(), String> {
    let res = reqwest::get(website.clone()).await;

    match res {
        Err(e) => Err(format!("{}", e)),
        Ok(response) => {
            if response.status().is_success() {
                Ok(())
            } else {
                Err(format!("Status {}", response.status().as_str()))
            }
        }
    }
}
