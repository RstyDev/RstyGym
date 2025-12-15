use crate::backend::domain::repositories::DayRepository;
use crate::backend::domain::services::DayService;
use crate::entities::{Day, Exercise};
use crate::utils::error::AppRes;

pub struct UpdateExercisesUseCase<T: DayRepository> {
    service: DayService<T>,
}

impl<T: DayRepository> UpdateExercisesUseCase<T> {
    pub fn new(service: T) -> Self {
        let service = DayService::new(service);
        Self { service }
    }

    pub async fn execute(&self, id: String, exercises: Vec<Exercise>) -> AppRes<Day> {
        self.service.update_exercises(id, exercises).await
    }
}