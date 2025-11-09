-- Add up migration script here
-- migrations/20251101033320_add_user_profile.up.sql
ALTER TABLE users ADD COLUMN display_name TEXT;
ALTER TABLE users ADD COLUMN avatar_url   TEXT;
ALTER TABLE users ADD COLUMN motto        TEXT;
