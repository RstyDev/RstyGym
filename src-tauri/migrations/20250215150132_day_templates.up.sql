-- Add up migration script here
CREATE TABLE IF NOT EXISTS 'day_templates' (
    'id' INTEGER NOT NULL,
    'routine' INTEGER NOT NULL,
    PRIMARY KEY ('id'),
    FOREIGN KEY ('routine') REFERENCES 'routines'('id')
);