use crate::backend::domain::repositories::RoutineRepository;
use crate::entities::Routine;
use crate::utils::error::AppRes;


#[derive(Clone)]
pub struct RoutineService<T: RoutineRepository> {
    book_repo: T,
}

impl <T: RoutineRepository> RoutineService<T> {
    pub fn new(book_repo: T) -> Self {
        Self{book_repo}
    }
    pub async fn get_all(&self) -> AppRes<Vec<Routine>> {
        self.book_repo.get_all().await
    }
    pub async fn get_by_device(&self, device: &str) -> AppRes<Vec<Routine>> {
        self.book_repo.get_by_device(device).await
    }
    pub async fn save(&self, device: String, routine: Routine) -> AppRes<()> {
        self.book_repo.save(device,routine).await
    }
    pub async fn delete(&self, id: &str) -> AppRes<()> {
        self.book_repo.delete(id).await
    }
    pub async fn update(&self, routine: &Routine) -> AppRes<Routine> {
        self.book_repo.update(routine).await
    }
}