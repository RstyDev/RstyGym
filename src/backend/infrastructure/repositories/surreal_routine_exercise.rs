use crate::{
    backend::{
        domain::repositories::RoutineRepository,
        infrastructure::db::{establish_connection, DBPool},
    },
    entities::Routine,
    error::{AppError, AppRes},
};
use std::sync::Arc;


#[derive(Clone)]
pub struct SurrealRoutineRepository {
    pool: DBPool,
}

impl SurrealRoutineRepository {
    pub async fn new() -> Self {
        Self {
            pool: establish_connection().await,
        }
    }
}
impl RoutineRepository for SurrealRoutineRepository {
    async fn save(&self, day: &Routine) -> AppRes<()> {
        todo!()
    }

    async fn delete(&self, id: &str) -> AppRes<()> {
        todo!()
    }

    async fn get_all(&self) -> AppRes<Vec<Routine>> {
        todo!()
    }

    async fn get_by_id(&self, id: &str) -> AppRes<Option<Routine>> {
        todo!()
    }

    async fn update(&self, persona: &Routine) -> AppRes<Routine> {
        todo!()
    }
}