CREATE TABLE video
(
    id          UUID PRIMARY KEY,
    name        VARCHAR(255) NOT NULL,
    description TEXT,
    channel     VARCHAR(255) NOT NULL
);

CREATE INDEX channel_index ON video (channel);


CREATE TABLE video_synthesis
(
    id         UUID PRIMARY KEY,
    video_id   UUID NOT NULL REFERENCES video (id),
    transcript TEXT,
    synthesis  TEXT
);

CREATE INDEX video_synthesis_index ON video_synthesis (video_id);

