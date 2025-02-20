use sqlx::{
    migrate::MigrateDatabase,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    Executor, Pool, Sqlite, SqlitePool,
};
use std::str::FromStr;
use structs::error::{AppError, AppRes as Res};
pub async fn db() -> Res<Pool<Sqlite>> {
    let url = "sqlite://sqlite.db";
    let mut exists = true;
    if !Sqlite::database_exists(url).await.unwrap_or(false) {
        match Sqlite::create_database(url).await {
            Ok(_) => {
                exists = false;
            }
            Err(error) => panic!("error: {}", error),
        }
    }

    let conn = match SqliteConnectOptions::from_str(url) {
        Ok(a) => a
            .journal_mode(SqliteJournalMode::Wal)
            .create_if_missing(true),
        Err(e) => {
            println!("{}", e);
            return Err(AppError::DBErr(url.to_string()));
        }
    }
    .journal_mode(SqliteJournalMode::Wal)
    .create_if_missing(true);

    let db = match SqlitePool::connect(url).await {
        Ok(o) => o,
        Err(e) => {
            println!("{e}");
            return Err(AppError::DBErr(url.to_string()));
        }
    };
    db.set_connect_options(conn);
    if !exists {
        fresh(&db).await;
    }
    Ok(db)
}

pub async fn fresh(db: &Pool<Sqlite>) {
    down(db).await;
    if let Err(e) = sqlx::query(QUERY).execute(db).await {
        println!("{}", e);
    };
}

pub async fn down(db: &Pool<Sqlite>) {
    db.execute(sqlx::query(
        r#"
    DROP TABLE IF EXISTS days;
    DROP TABLE IF EXISTS exercises;
    DROP TABLE IF EXISTS series;
    DROP TABLE IF EXISTS weeks;
    DROP TABLE IF EXISTS day_templates;
    DROP TABLE IF EXISTS routines;
    "#,
    ))
    .await
    .unwrap();
}

const QUERY: &str = r#"
CREATE TABLE IF NOT EXISTS 'days' (
    'id' INTEGER NOT NULL,
    'state' TEXT NOT NULL,
    'date' DATE NOT NULL,
    'week' INTEGER NOT NULL,
    PRIMARY KEY ('id'),
    FOREIGN KEY ('week') REFERENCES 'weeks'('id')
);
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
CREATE TABLE IF NOT EXISTS 'series' (
    'id' INTEGER NOT NULL,
    'exercise' INTEGER NOT NULL,
    'count' INTEGER NOT NULL,
    'weight' REAL,
    PRIMARY KEY ('id'),
    FOREIGN KEY ('exercise') REFERENCES 'exercises'('id')
);
CREATE TABLE IF NOT EXISTS 'weeks' (
    'id' INTEGER NOT NULL,
    'completed' BOOLEAN NOT NULL,
    'routine' INTEGER NOT NULL,
    PRIMARY KEY ('id'),
    FOREIGN KEY ('routine') REFERENCES 'routines'('id')
);
CREATE TABLE IF NOT EXISTS 'day_templates' (
    'id' INTEGER NOT NULL,
    'routine' INTEGER NOT NULL,
    PRIMARY KEY ('id'),
    FOREIGN KEY ('routine') REFERENCES 'routines'('id')
);
CREATE TABLE IF NOT EXISTS 'routines' (
    'id' INTEGER NOT NULL,
    'last_check_in' DATE NOT NULL,
    'last_day_index' INTEGER NOT NULL,
    'created_by' TEXT,
    'created_at' DATE NOT NULL,
    PRIMARY KEY ('id')
);
 "#;
