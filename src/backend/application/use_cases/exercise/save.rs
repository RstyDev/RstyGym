use crate::backend::domain::repositories::ExerciseRepository;
use crate::backend::domain::services::ExerciseService;
use crate::entities::Exercise;
use crate::utils::error::AppRes;

pub struct SaveExerciseUseCase<T: ExerciseRepository> {
    service: ExerciseService<T>,
}

impl<T: ExerciseRepository> SaveExerciseUseCase<T> {
    pub fn new(service: T) -> Self {
        let service = ExerciseService::new(service);
        Self { service }
    }

    pub async fn execute(&self, exercise: Exercise) -> AppRes<()> {
        self.service.save(exercise).await
    }
}