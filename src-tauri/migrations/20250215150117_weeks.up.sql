-- Add up migration script here
CREATE TABLE IF NOT EXISTS 'weeks' (
    'id' INTEGER NOT NULL,
    'completed' BOOLEAN NOT NULL,
    'routine' INTEGER NOT NULL,
    PRIMARY KEY ('id'),
    FOREIGN KEY ('routine') REFERENCES 'routines'('id')
);