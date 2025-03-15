CREATE TABLE prompts
(
    id   UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    prompt TEXT
);