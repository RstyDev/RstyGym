#![allow(async_fn_in_trait)]
use crate::{entities::Routine, error::AppRes};

pub trait RoutineRepository {
    async fn save(&self, routine: Routine) -> AppRes<()>;
    async fn delete(&self, id: &str) -> AppRes<()>;
    async fn get_all(&self) -> AppRes<Vec<Routine>>;
    async fn get_by_user(&self, id: &str) -> AppRes<Vec<Routine>>;
    async fn update(&self, routine: &Routine) -> AppRes<Routine>;
}