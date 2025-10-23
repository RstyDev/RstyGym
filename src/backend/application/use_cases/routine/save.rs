use crate::backend::domain::repositories::RoutineRepository;
use crate::backend::domain::services::RoutineService;
use crate::entities::Routine;
use crate::utils::error::AppRes;

pub struct SaveRoutineUseCase<T: RoutineRepository> {
    service: RoutineService<T>,
}

impl<T: RoutineRepository> SaveRoutineUseCase<T> {
    pub fn new(service: T) -> Self {
        let service = RoutineService::new(service);
        Self { service }
    }

    pub async fn execute(&self, device: String, routine: Routine) -> AppRes<()> {
        self.service.save(device,routine).await
    }
}