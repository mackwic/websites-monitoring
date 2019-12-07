use diesel::pg::PgConnection;
use diesel::prelude::*;
use log::{debug, error};
use std::env;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::schema::*;
use crate::Messages;

#[derive(diesel::Insertable)]
#[table_name = "crawling_session"]
struct NewCrawlResult {
    start_date: chrono::DateTime<chrono::Local>,
    url: String,
    is_success: bool,
    duration: diesel::pg::data_types::PgInterval,
}

pub fn spawn_receiver(mut receiver: Receiver<Messages>, mut sender_end_channel: Sender<()>) {
    tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            match message {
                Messages::EndIteration => break,
                Messages::EndPingSite(timestamp, url, result, duration) => {
                    store_result(timestamp, url, result, duration)
                }
                _ => debug!("got {:?}", message),
            }
        }
        sender_end_channel
            .send(())
            .await
            .expect("error when sending EndIteration acknowledge");
    });
}

fn store_result(
    timestamp: chrono::DateTime<chrono::Local>,
    url: reqwest::Url,
    result: Result<(), String>,
    duration: chrono::Duration,
) {
    let insertable_record = NewCrawlResult {
        start_date: timestamp,
        url: url.into_string(),
        is_success: result.is_ok(),
        duration: diesel::pg::data_types::PgInterval::from_microseconds(
            duration.num_milliseconds(),
        ),
    };

    let connection = establish_connection();
    let result = diesel::insert_into(crawling_session::table)
        .values(insertable_record)
        .execute(&connection);

    if let Err(err) = result {
        error!("Unable to insert record: {}", err)
    }
}

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("Error connecting to database")
}
