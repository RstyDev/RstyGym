use crate::backend::domain::repositories::ExerciseRepository;
use crate::entities::Exercise;
use crate::error::AppRes;

#[derive(Clone)]
pub struct ExerciseService<T: ExerciseRepository> {
    book_repo: T,
}

impl <T: ExerciseRepository> ExerciseService<T> {
    pub fn new(book_repo: T) -> Self {
        Self{book_repo}
    }
    pub async fn get_all(&self) -> AppRes<Vec<Exercise>> {
        self.book_repo.get_all().await
    }
    pub async fn get_by_id(&self, id: &str) -> AppRes<Option<Exercise>> {
        self.book_repo.get_by_id(id).await
    }
    pub async fn get_by_user(&self, user: &str) -> AppRes<Vec<Exercise>> {
        self.book_repo.get_by_user(user).await
    }
    pub async fn save(&self, exercise: &Exercise) -> AppRes<()> {
        self.book_repo.save(exercise).await
    }
    pub async fn delete(&self, id: &str) -> AppRes<()> {
        self.book_repo.delete(id).await
    }
    pub async fn update(&self, exercise: &Exercise) -> AppRes<Exercise> {
        self.book_repo.update(exercise).await
    }
}