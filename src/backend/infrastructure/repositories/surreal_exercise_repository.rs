use crate::{
    backend::{
        domain::repositories::ExerciseRepository,
        infrastructure::db::{establish_connection, DBPool},
    },
    entities::Exercise,
    error::{AppError, AppRes},
};
use std::sync::Arc;


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
impl ExerciseRepository for SurrealExerciseRepository {
    async fn save(&self, exercise: &Exercise) -> AppRes<()> {
        todo!()
    }

    async fn delete(&self, id: &str) -> AppRes<()> {
        todo!()
    }

    async fn get_all(&self) -> AppRes<Vec<Exercise>> {
        todo!()
    }

    async fn get_by_id(&self, id: &str) -> AppRes<Option<Exercise>> {
        todo!()
    }

    async fn update(&self, exercise: &Exercise) -> AppRes<Exercise> {
        todo!()
    }
}