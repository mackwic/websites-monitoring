table! {
    crawling_session (id) {
        id -> Int4,
        start_date -> Timestamptz,
        url -> Varchar,
        is_success -> Bool,
        duration_ms -> Int4,
        error_description -> Nullable<Varchar>,
    }
}
