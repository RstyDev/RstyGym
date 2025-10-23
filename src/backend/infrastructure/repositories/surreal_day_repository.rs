use crate::{backend::{
    domain::repositories::DayRepository,
    infrastructure::db::{establish_connection, DBPool},
}, day_db, entities::Day, record_id};
use crate::backend::infrastructure::db::DayDB;
use std::sync::Arc;
use surrealdb::RecordId;
use crate::entities::Exercise;
use crate::utils::error::{AppError, AppRes};

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
    async fn save(&self, device: String, day: Day) -> AppRes<()> {
        let day = day_db!(day);
        // let res = self.pool.insert(&day).await;
        let res = self
            .pool
            .query(
                r#"
        insert into days {
            device: $device,
            state: $state,
            date: $date,
            exercises: $exercises,
        }
        "#,
            )
            .bind(("device",record_id!("device",device)))
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

    async fn get_by_device(&self, id: &str) -> AppRes<Vec<Day>> {
        Err(AppError::IndexErr(1))
    }


    async fn update(&self, day: &Day) -> AppRes<Day> {
        Err(AppError::IndexErr(1))
    }
    async fn update_exercises(&self, id: String, exercises: Vec<Exercise>) -> AppRes<Day> {

        let res = self.pool.query(r#"
            UPDATE $id CONTENT {
                exercises: $exercises,
            }
        "#).bind(("id",record_id!("days",id)))
            .bind(("exercises",exercises))
            .await;
        match res {
            Ok(mut res) => {
                let record: Option<Day> = res.take(0usize).unwrap();
                println!("Res: {:?}", record);
                if let Some(rec) = record {
                    Ok(rec)
                } else {
                    Err(AppError::NotFound(93))
                }
            },
            Err(e) => Err(AppError::ValidationErr(96,e.to_string())),
        }
    }
}