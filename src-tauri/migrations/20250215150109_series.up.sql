-- Add up migration script here
CREATE TABLE IF NOT EXISTS 'series' (
    'id' INTEGER NOT NULL,
    'exercise' INTEGER NOT NULL,
    'count' INTEGER NOT NULL,
    'weight' REAL,
    PRIMARY KEY ('id'),
    FOREIGN KEY ('exercise') REFERENCES 'exercises'('id')
);