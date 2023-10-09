-- Add migration script here
CREATE TABLE IF NOT EXISTS quotes
(
    id         uuid PRIMARY KEY,
    book       varchar     NOT NULL,
    quote      text        NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL,
    UNIQUE (book, quote)
)
