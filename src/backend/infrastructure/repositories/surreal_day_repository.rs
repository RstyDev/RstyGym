use crate::{
    backend::{
        domain::repositories::DayRepository,
        infrastructure::db::{establish_connection, DBPool},
    },
    entities::Day,
    error::{AppError, AppRes},
};
use std::sync::Arc;
use surrealdb::sql::thing;

#[derive(Clone)]
pub struct SurrealDayRepository {
    pool: DBPool,
}

impl SurrealDayRepository {
    pub async fn new() -> Self {
        Self {
            pool: establish_connection().await,
        }
    }
}
/*
pub struct Day {
    id: Option<String>,
    state: DayState,
    date: NaiveDate,
    exercises: Vec<Exercise>,
}*/
impl DayRepository for SurrealDayRepository {
    async fn save(&self, day: &Day) -> AppRes<()> {
        let res = self
            .pool
            .query(
                r#"
        insert into days {
            state: $day.state,
            date: $day.date,
            exercises: $exercises,
        }
        "#,
            )
            .bind(("day", day.to_owned()))
            .bind(("exercises",day.exercises().into_iter().map(|e|thing(&e.id().as_ref().unwrap_or_default()))))
            .await;
        match res {
            Ok(a) => {
                println!("{:#?}", a);
                Ok(())
            }
            Err(e) => Err(AppError::DBErr(56, e.to_string())),
        }
    }

    async fn delete(&self, id: &str) -> AppRes<()> {
        todo!()
    }

    async fn get_all(&self) -> AppRes<Vec<Day>> {
        todo!()
    }

    async fn get_by_id(&self, id: &str) -> AppRes<Option<Day>> {
        todo!()
    }

    async fn update(&self, persona: &Day) -> AppRes<Day> {
        todo!()
    }
}