#![allow(async_fn_in_trait)]
use crate::entities::Day;
use crate::entities::Exercise;
use crate::utils::error::AppRes;

pub trait DayRepository {
    async fn save(&self, day: Day) -> AppRes<()>;
    async fn delete(&self, id: &str) -> AppRes<()>;
    async fn get_all(&self) -> AppRes<Vec<Day>>;
    async fn get_by_user(&self, id: &str) -> AppRes<Vec<Day>>;
    async fn update(&self, day: &Day) -> AppRes<Day>;
    async fn update_exercises(&self,id: String, exercises: Vec<Exercise>) -> AppRes<Day>;
}