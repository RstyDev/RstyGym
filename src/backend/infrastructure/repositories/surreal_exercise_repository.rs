use std::sync::Arc;
use crate::{backend::{
    domain::repositories::ExerciseRepository,
    infrastructure::db::{establish_connection, DBPool},
}, entities::Exercise, exercise_db, record_id};
use crate::utils::error::{AppError, AppRes};

#[derive(Clone)]
pub struct SurrealExerciseRepository {
    pool: DBPool,
}

impl SurrealExerciseRepository {
    pub async fn new() -> Self {
        Self {
            pool: establish_connection().await,
        }
    }
}
/*
    id: Option<String>,
    name: String,
    series: [Option<Series>; 4],
    group: MuscleGroup,*/
impl ExerciseRepository for Arc<SurrealExerciseRepository> {
    async fn save(&self, device: String, exercise: Exercise) -> AppRes<()> {
        let exercise = exercise_db!(exercise);
        // let res = self.pool.insert(&exercise).await;
        let res = self
            .pool
            .query(
                r#"
        insert into exercises {
            device: $device,
            name: $name,
            series: $series,
            group: $group,
        }
        "#,
            )
            .bind(("device", record_id!("device",device)))
            .bind(("name", exercise.name().to_owned()))
            .bind(("series", exercise.series().to_owned()))
            .bind(("group",exercise.group()))
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

    async fn get_all(&self) -> AppRes<Vec<Exercise>> {
        Err(AppError::IndexErr(1))
    }

    async fn get_by_device(&self, id: &str) -> AppRes<Vec<Exercise>> {
        Err(AppError::IndexErr(1))
    }

    async fn update(&self, exercise: &Exercise) -> AppRes<Exercise> {
        Err(AppError::IndexErr(1))
    }
}