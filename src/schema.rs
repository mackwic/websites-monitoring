table! {
    crawling_session (id) {
        id -> Int4,
        start_date -> Timestamp,
        url -> Varchar,
        is_success -> Bool,
        duration -> Interval,
    }
}
