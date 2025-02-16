-- Add up migration script here
CREATE TABLE IF NOT EXISTS 'routines' (
    'id' INTEGER NOT NULL,
    'last_check_in' DATE NOT NULL,
    'last_day_index' INTEGER NOT NULL,
    'created_by' TEXT,
    'created_at' DATE NOT NULL,
    PRIMARY KEY ('id')
);