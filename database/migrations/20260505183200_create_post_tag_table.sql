CREATE TABLE IF NOT EXISTS post_tag (
    post_id    INTEGER NOT NULL,
    tag_id     INTEGER NOT NULL,
    PRIMARY KEY (post_id, tag_id)
);
