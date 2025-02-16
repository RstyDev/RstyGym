-- Add up migration script here
CREATE TABLE IF NOT EXISTS 'days' (
    'id' INTEGER NOT NULL,
    'state' TEXT NOT NULL,
    'date' DATE NOT NULL,
    'week' INTEGER NOT NULL,
    PRIMARY KEY ('id'),
    FOREIGN KEY ('week') REFERENCES 'weeks'('id')
);