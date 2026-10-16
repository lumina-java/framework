-- Migration: Create Jobs Table for Persistent Queue
-- Compatible with MySQL, PostgreSQL, and SQLite
CREATE TABLE IF NOT EXISTS jobs (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    queue        VARCHAR(255)    NOT NULL DEFAULT 'default',
    payload      TEXT            NOT NULL,
    attempts     INTEGER         NOT NULL DEFAULT 0,
    reserved_at  DATETIME        DEFAULT NULL,
    available_at DATETIME        NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at   DATETIME        NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_jobs_queue_available ON jobs(queue, available_at);
