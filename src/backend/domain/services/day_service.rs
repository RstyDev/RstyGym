use crate::backend::domain::repositories::DayRepository;
use crate::entities::Day;
use crate::error::AppRes;

#[derive(Clone)]
pub struct DayService<T: DayRepository> {
    book_repo: T,
}

impl <T: DayRepository> DayService<T> {
    pub fn new(book_repo: T) -> Self {
        Self{book_repo}
    }
    pub async fn get_all(&self) -> AppRes<Vec<Day>> {
        self.book_repo.get_all().await
    }
    pub async fn get_by_user(&self, user: &str) -> AppRes<Vec<Day>> {
        self.book_repo.get_by_user(user).await
    }
    pub async fn save(&self, day: Day) -> AppRes<()> {
        self.book_repo.save(day).await
    }
    pub async fn delete(&self, id: &str) -> AppRes<()> {
        self.book_repo.delete(id).await
    }
    pub async fn update(&self, day: &Day) -> AppRes<Day> {
        self.book_repo.update(day).await
    }
}