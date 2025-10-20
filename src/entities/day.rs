use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use crate::entities::Exercise;


#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq)]
pub enum DayState {
    #[default]
    Free,
    Checked,
    Complete,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Day {
    id: Option<String>,
    state: DayState,
    date: NaiveDate,
    exercises: Vec<Exercise>,
}



impl Display for DayState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DayState::Free => f.write_fmt(format_args!("Free")),
            DayState::Checked => f.write_fmt(format_args!("Checked")),
            DayState::Complete => f.write_fmt(format_args!("Complete")),
        }
    }
}
impl From<String> for DayState {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Free" => DayState::Free,
            "Checked" => DayState::Checked,
            "Complete" => DayState::Complete,
            _ => DayState::Free,
        }
    }
}

impl Day {
    pub fn exercises(&self) -> &Vec<Exercise> {
        &self.exercises
    }
}