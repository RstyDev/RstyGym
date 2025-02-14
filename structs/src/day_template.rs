use serde::{Deserialize, Serialize};
use crate::{exercise::Exercise, error::AppRes as Res};
use crate::error::AppError;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DayTemplate {
    id: i64,
    exercises: Vec<Exercise>
}

impl DayTemplate {
    pub fn build(id: Option<i64>, exercises: Vec<Exercise>) -> DayTemplate {
        DayTemplate { id: id.unwrap_or_default(), exercises }
    }
    pub fn id(&self) -> &i64 {
        &self.id
    }
    pub fn exercises(&self) -> &Vec<Exercise> {
        &self.exercises
    }
    pub fn exercise_at(&self, index: usize) -> Res<&Exercise> {
        if self.exercises.len() > &index {
            Ok(&self.exercises[index])
        }else{
            Err(AppError::IndexErr)
        }

    }
}