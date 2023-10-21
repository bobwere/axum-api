-- Add migration script here
CREATE TABLE IF NOT EXISTS quotes (
    id UUID PRIMARY KEY,
    book varchar NOT NULL,
    quote TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    upcdated_at TIMESTAMPTZ NOT NULL,
    UNIQUE (book, quote)
);