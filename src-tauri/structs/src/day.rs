use std::fmt::{Display, Formatter, Write};
use chrono::NaiveDate;
use crate::{exercise::Exercise, error::AppRes as Res};
use serde::{Deserialize, Serialize};
use crate::error::AppError;

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub enum DayState {
    #[default]
    Free,
    Checked,
    Complete,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Day {
    id: i64,
    state: DayState,
    date: NaiveDate,
    exercises: Vec<Exercise>,
}

impl Day {
    pub fn build(id: Option<i64>, state: DayState, date: NaiveDate, exercises: Vec<Exercise>) -> Day {
        Day {
            id: id.unwrap_or_default(),
            state,
            date,
            exercises,
        }
    }
    pub fn id(&self) -> &i64 {
        &self.id
    }
    pub fn set_id(&mut self, id: i64) {
        self.id = id;
    }
    pub fn state(&self) -> DayState {
        self.state
    }
    pub fn set_state(&mut self, state: DayState) {
        self.state = state;
    }
    pub fn date(&self) -> &NaiveDate {
        &self.date
    }
    pub fn set_date(&mut self, date: NaiveDate) {
        self.date = date;
    }
    pub fn exercises(&self) -> &Vec<Exercise> {
        &self.exercises
    }
    pub fn exercise_at(&self, index: usize) -> Res<&Exercise> {
        if &self.exercises.len() > &index {
            Ok(&self.exercises[index])
        }else{
            Err(AppError::IndexErr)
        }
    }
    pub fn exercises_mut(&mut self) -> &mut Vec<Exercise> {
        &mut self.exercises
    }
    pub fn exercise_at_mut(&mut self, index: usize) -> Res<&mut Exercise> {
        if &self.exercises.len() > &index {
            Ok(&mut self.exercises[index])
        }else{
            Err(AppError::IndexErr)
        }
    }
    pub fn set_exercises(&mut self, exercises: Vec<Exercise>) {
        self.exercises = exercises;
    }
    pub fn set_exercise_at(&mut self, exercise: Exercise, index: usize) -> Res<()> {
        if &self.exercises.len() > &index {
            self.exercises[index] = exercise;
            Ok(())
        }else{Err(AppError::IndexErr)}
    }
}

impl Display for DayState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            DayState::Free => f.write_fmt(format_args!("Free")),
            DayState::Checked => f.write_fmt(format_args!("Checked")),
            DayState::Complete => f.write_fmt(format_args!("Complete")),
        }
    }
}
impl From<String> for DayState{
    fn from(value: String) -> Self {
        match value.as_str() {
            "Free" => DayState::Free,
            "Checked" => DayState::Checked,
            "Complete" => DayState::Complete,
            _ => DayState::Free,
        }
    }
}