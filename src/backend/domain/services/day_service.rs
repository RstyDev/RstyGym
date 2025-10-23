use crate::backend::domain::repositories::DayRepository;
use crate::entities::{Day, Exercise};
use crate::utils::error::AppRes;

#[derive(Clone)]
pub struct DayService<T: DayRepository> {
    repo: T,
}

impl <T: DayRepository> DayService<T> {
    pub fn new(repo: T) -> Self {
        Self{repo}
    }
    pub async fn get_all(&self) -> AppRes<Vec<Day>> {
        self.repo.get_all().await
    }
    pub async fn get_by_device(&self, device: &str) -> AppRes<Vec<Day>> {
        self.repo.get_by_device(device).await
    }
    pub async fn save(&self, device: String, day: Day) -> AppRes<()> {
        self.repo.save(device,day).await
    }
    pub async fn delete(&self, id: &str) -> AppRes<()> {
        self.repo.delete(id).await
    }
    pub async fn update(&self, day: &Day) -> AppRes<Day> {
        self.repo.update(day).await
    }

    pub async fn update_exercises(&self, id: String, exercises: Vec<Exercise>) -> AppRes<Day> { self.repo.update_exercises(id, exercises).await }
}