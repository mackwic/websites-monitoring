#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use reqwest;

mod logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::set_up_logging();
    info!("start of run");

    {
        let websites = vec!["https://scalingo.com"];

        for site in websites {
            fetch_site(site).await;
        }
    };

    info!("end of run");
    Ok(())
}

async fn fetch_site(website: &str) {
    debug!("Fetching site: {}", website);

    let res = reqwest::get(website).await;

    match res {
        Err(e) => error!("Error: {}", e),
        Ok(res) => info!("Success: {:?}", res.status()),
    }
}
