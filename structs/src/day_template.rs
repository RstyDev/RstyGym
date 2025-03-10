use crate::{
    error::{AppError, AppRes as Res},
    exercise::Exercise,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DayTemplate {
    id: i64,
    exercises: Vec<Exercise>,
}

impl DayTemplate {
    pub fn build(id: Option<i64>, exercises: Vec<Exercise>) -> DayTemplate {
        DayTemplate {
            id: id.unwrap_or_default(),
            exercises,
        }
    }
    pub fn id(&self) -> &i64 {
        &self.id
    }

    pub fn set_id(&mut self, id: i64) {
        self.id = id;
    }
    pub fn exercises(&self) -> &Vec<Exercise> {
        &self.exercises
    }
    pub fn exercises_mut(&mut self) -> &mut Vec<Exercise> {
        &mut self.exercises
    }
    pub fn exercise_at(&self, index: usize) -> Res<&Exercise> {
        if &self.exercises.len() > &index {
            Ok(&self.exercises[index])
        } else {
            Err(AppError::IndexErr(37))
        }
    }
    pub fn exercise_at_mut(&mut self, index: usize) -> Res<&mut Exercise> {
        if self.exercises.len() > index {
            Ok(&mut self.exercises[index])
        } else {
            Err(AppError::IndexErr(44))
        }
    }
}
