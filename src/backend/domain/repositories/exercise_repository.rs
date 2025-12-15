#![allow(async_fn_in_trait)]
use crate::entities::Exercise;
use crate::utils::error::AppRes;

pub trait ExerciseRepository {
    async fn save(&self, device: String, exercise: Exercise) -> AppRes<()>;
    async fn delete(&self, id: &str) -> AppRes<()>;
    async fn get_all(&self) -> AppRes<Vec<Exercise>>;
    async fn get_by_device(&self, device: &str) -> AppRes<Vec<Exercise>>;
    async fn update(&self, exercise: &Exercise) -> AppRes<Exercise>;
}