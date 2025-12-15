#![allow(async_fn_in_trait)]
use crate::entities::Routine;
use crate::utils::error::AppRes;

pub trait RoutineRepository {
    async fn save(&self, device: String, routine: Routine) -> AppRes<()>;
    async fn delete(&self, id: &str) -> AppRes<()>;
    async fn get_all(&self) -> AppRes<Vec<Routine>>;
    async fn get_by_device(&self, device: &str) -> AppRes<Vec<Routine>>;
    async fn update(&self, routine: &Routine) -> AppRes<Routine>;
}