CREATE TABLE websites_to_crawl(
    url varchar PRIMARY KEY,
    is_enabled boolean NOT NULL DEFAULT true,
    created_at timestamp with time zone NOT NULL DEFAULT now()
);

