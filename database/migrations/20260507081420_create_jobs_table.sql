-- Migration: Create Jobs Table for Persistent Queue
-- Compatible with MySQL, PostgreSQL, and SQLite
CREATE TABLE IF NOT EXISTS jobs (
    id           BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    queue        VARCHAR(255)    NOT NULL DEFAULT 'default',
    payload      LONGTEXT        NOT NULL,
    attempts     TINYINT         NOT NULL DEFAULT 0,
    reserved_at  DATETIME        DEFAULT NULL,
    available_at DATETIME        NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at   DATETIME        NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_jobs_queue_available ON jobs(queue, available_at);
