#![allow(async_fn_in_trait)]
use crate::{entities::Exercise, error::AppRes};

pub trait ExerciseRepository {
    async fn save(&self, exercise: Exercise) -> AppRes<()>;
    async fn delete(&self, id: &str) -> AppRes<()>;
    async fn get_all(&self) -> AppRes<Vec<Exercise>>;
    async fn get_by_user(&self, id: &str) -> AppRes<Vec<Exercise>>;
    async fn update(&self, exercise: &Exercise) -> AppRes<Exercise>;
}