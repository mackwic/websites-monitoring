use diesel::pg::PgConnection;
use diesel::prelude::*;
use log::error;
use std::env;

use crate::schema::{self, *};

#[derive(diesel::Insertable)]
#[table_name = "crawling_session"]
struct NewCrawlResult {
    start_date: chrono::DateTime<chrono::Local>,
    url: String,
    is_success: bool,
    duration_ms: i32,
    error_description: Option<String>,
}

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("Error connecting to database")
}

pub fn fetch_websites_to_crawl() -> Result<Vec<String>, String> {
    use schema::websites_to_crawl::dsl::*;

    let connection = establish_connection();
    let result = websites_to_crawl
        .select(url)
        .filter(is_enabled.eq(true))
        .load::<(String)>(&connection)
        .or_else(|err| Err(format!("Error when loading websites to crawl: {}", err)))?;
    Ok(result) // FIXME handle errors with a log
}

pub fn store_crawl_result(
    timestamp: chrono::DateTime<chrono::Local>,
    url: reqwest::Url,
    result: Result<(), String>,
    duration: chrono::Duration,
) {
    let insertable_record = NewCrawlResult {
        start_date: timestamp,
        url: url.into_string(),
        is_success: result.is_ok(),
        duration_ms: duration.num_milliseconds() as i32,
        error_description: result.err(),
    };

    let connection = establish_connection();
    let result = diesel::insert_into(crawling_session::table)
        .values(insertable_record)
        .execute(&connection);

    if let Err(err) = result {
        error!("Unable to insert record: {}", err)
    }
}
