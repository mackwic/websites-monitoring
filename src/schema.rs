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

table! {
    websites_to_crawl (url) {
        url -> Varchar,
        is_enabled -> Bool,
        created_at -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    crawling_session,
    websites_to_crawl,
);
