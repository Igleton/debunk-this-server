CREATE TYPE transcript AS (
    text VARCHAR,
    lang VARCHAR,
    duration INT,
    "offset" INT
    );

CREATE TABLE video
(
    id           VARCHAR(255) PRIMARY KEY,
    name         VARCHAR(255) NOT NULL,
    description  TEXT,
    channel      VARCHAR(255),
    channel_name VARCHAR(255)
);

CREATE INDEX channel_index ON video (channel);


CREATE TABLE video_synthesis
(
    id           UUID DEFAULT gen_random_uuid(),
    created_date TIMESTAMP,
    video_id     UUID NOT NULL REFERENCES video (id),
    transcript   transcript[],
    synthesis    TEXT,
    PRIMARY KEY (id, created_date)
);

CREATE INDEX video_synthesis_index ON video_synthesis (video_id);

CREATE TABLE _processings
(
    id UUID PRIMARY KEY,
    video_id VARCHAR(255) NOT NULL,
    status VARCHAR(16),
);

CREATE TABLE prompts
(
    id   UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT now(),
    prompt TEXT
);
