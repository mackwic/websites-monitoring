
CREATE TABLE crawling_session(
    id SERIAL PRIMARY KEY,
    start_date timestamp NOT NULL,
    url character varying NOT NULL,
    is_success boolean NOT NULL,
    duration interval NOT NULL
);

CREATE INDEX index_start_date_on_crawling_session
ON crawling_session(
    start_date DESC
);


CREATE INDEX index_url_on_crawling_session
ON crawling_session(
    url
);