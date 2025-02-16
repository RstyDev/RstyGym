-- Add up migration script here
CREATE TABLE IF NOT EXISTS 'exercises' (
    'id' INTEGER NOT NULL,
    'name' TEXT NOT NULL,
    'group' TEXT NOT NULL,
    'day' INTEGER,
    'day_template' INTEGER,
    PRIMARY KEY ('id'),
    FOREIGN KEY ('day') REFERENCES 'days'('id'),
    FOREIGN KEY ('day_template') REFERENCES 'day_templates'('id')
);