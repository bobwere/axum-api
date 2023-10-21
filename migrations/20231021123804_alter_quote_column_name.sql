-- Add migration script here
ALTER TABLE
    quotes RENAME COLUMN upcdated_at TO updated_at;