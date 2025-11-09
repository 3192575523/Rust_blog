-- Add up migration script here
-- posts
CREATE TABLE IF NOT EXISTS posts (
  id            TEXT PRIMARY KEY,
  slug          TEXT NOT NULL UNIQUE,
  title         TEXT NOT NULL,
  excerpt       TEXT,
  body_md       TEXT NOT NULL,
  body_html     TEXT NOT NULL,
  status        TEXT NOT NULL CHECK(status IN ('draft','published')),
  visibility    TEXT NOT NULL CHECK(visibility IN ('public','private')) DEFAULT 'public',
  author_id     TEXT NOT NULL,
  published_at  TEXT,
  created_at    TEXT NOT NULL,
  updated_at    TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_posts_status_visibility_pubat
  ON posts(status, visibility, published_at DESC);
CREATE INDEX IF NOT EXISTS idx_posts_slug ON posts(slug);

-- tags / post_tags
CREATE TABLE IF NOT EXISTS tags(
  id    TEXT PRIMARY KEY,
  slug  TEXT NOT NULL UNIQUE,
  name  TEXT NOT NULL UNIQUE
);
CREATE TABLE IF NOT EXISTS post_tags(
  post_id TEXT NOT NULL,
  tag_id  TEXT NOT NULL,
  PRIMARY KEY(post_id, tag_id),
  FOREIGN KEY(post_id) REFERENCES posts(id) ON DELETE CASCADE,
  FOREIGN KEY(tag_id)  REFERENCES tags(id)  ON DELETE CASCADE
);

-- users
CREATE TABLE IF NOT EXISTS users(
  id            TEXT PRIMARY KEY,
  username      TEXT NOT NULL UNIQUE,
  password_hash TEXT NOT NULL,
  created_at    TEXT NOT NULL
);