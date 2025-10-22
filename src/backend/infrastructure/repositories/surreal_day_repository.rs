use crate::{
    backend::{
        domain::repositories::DayRepository,
        infrastructure::db::{establish_connection, DBPool},
    },
    entities::Day,
    error::{AppError, AppRes},
};
use crate::backend::infrastructure::db::DayDB;
use std::sync::Arc;

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
impl DayRepository for Arc<SurrealDayRepository> {
    async fn save(&self, day: Day) -> AppRes<()> {
        let day = DayDB::from(day);
        // let res = self.pool.insert(&day).await;
        let res = self
            .pool
            .query(
                r#"
        insert into days {
            state: $state,
            date: $date,
            exercises: $exercises,
        }
        "#,
            )
            .bind(("state", day.state()))
            .bind(("date", day.date()))
            .bind(("exercises",day.exercises().to_owned()))
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
        Err(AppError::IndexErr(1))
    }

    async fn get_all(&self) -> AppRes<Vec<Day>> {
        Err(AppError::IndexErr(1))
    }

    async fn get_by_user(&self, id: &str) -> AppRes<Vec<Day>> {
        Err(AppError::IndexErr(1))
    }


    async fn update(&self, day: &Day) -> AppRes<Day> {
        Err(AppError::IndexErr(1))
    }
}