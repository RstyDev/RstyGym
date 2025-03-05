-- Add up migration script here
CREATE TABLE IF NOT EXISTS 'exercises' (
    'id' INTEGER NOT NULL,
    'name' TEXT NOT NULL,
    'muscle_group' TEXT NOT NULL,
    'day' INTEGER,
    'day_template' INTEGER,
    PRIMARY KEY ('id')
);