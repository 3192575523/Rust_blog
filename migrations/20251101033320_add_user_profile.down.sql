-- Add down migration script here
-- migrations/20251101033320_add_user_profile.down.sql
BEGIN TRANSACTION;

-- 1) 建一个只包含原始列的新表
CREATE TABLE users_tmp(
  id            TEXT PRIMARY KEY,
  username      TEXT NOT NULL UNIQUE,
  password_hash TEXT NOT NULL,
  created_at    TEXT NOT NULL
);

-- 2) 拷贝旧表中对应数据
INSERT INTO users_tmp (id, username, password_hash, created_at)
SELECT id, username, password_hash, created_at FROM users;

-- 3) 用新表替换旧表
DROP TABLE users;
ALTER TABLE users_tmp RENAME TO users;

COMMIT;
