
CREATE TABLE crawling_session(
    id SERIAL PRIMARY KEY,
    start_date timestamp with time zone NOT NULL,
    url character varying NOT NULL,
    is_success boolean NOT NULL,
    duration_ms integer NOT NULL,
    error_description varchar
);

CREATE INDEX index_start_date_on_crawling_session
ON crawling_session(
    start_date DESC
);


CREATE INDEX index_url_on_crawling_session
ON crawling_session(
    url
);

CREATE VIEW v_crawling_session_metabase AS
SELECT
    CAST(EXTRACT(EPOCH FROM "start_date") AS integer) as "start_date_ts",
    "start_date",
    "duration_ms",
    "is_success",
    "url",
    "error_description"
FROM crawling_session
ORDER BY start_date_ts ASC;