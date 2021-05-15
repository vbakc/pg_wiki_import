-- Add migration script here

CREATE TABLE articles (
    id serial PRIMARY KEY NOT NULL,
    title text NOT NULL,
    content text NOT NULL
);
CREATE INDEX articles_pk_idx ON articles (id);
