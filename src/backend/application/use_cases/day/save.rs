use crate::backend::domain::repositories::DayRepository;
use crate::backend::domain::services::DayService;
use crate::entities::Day;
use crate::utils::error::AppRes;

pub struct SaveDayUseCase<T: DayRepository> {
    service: DayService<T>,
}

impl<T: DayRepository> SaveDayUseCase<T> {
    pub fn new(service: T) -> Self {
        let service = DayService::new(service);
        Self { service }
    }

    pub async fn execute(&self, device: String, day: Day) -> AppRes<()> {
        self.service.save(device,day).await
    }
}