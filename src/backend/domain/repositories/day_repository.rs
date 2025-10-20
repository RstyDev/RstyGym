#![allow(async_fn_in_trait)]
use crate::{entities::Day, error::AppRes};

pub trait DayRepository {
    async fn save(&self, day: &Day) -> AppRes<()>;
    async fn delete(&self, id: &str) -> AppRes<()>;
    async fn get_all(&self) -> AppRes<Vec<Day>>;
    async fn get_by_id(&self, id: &str) -> AppRes<Option<Day>>;
    async fn update(&self, day: &Day) -> AppRes<Day>;
}