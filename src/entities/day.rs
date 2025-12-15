use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
#[cfg(feature = "ssr")]
use surrealdb::RecordId;
#[cfg(feature = "ssr")]
use crate::backend::infrastructure::db::DayDB;
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
    pub fn new(id: Option<String>, state: DayState, date: NaiveDate, exercises: Vec<Exercise>) -> Self {
        Self { id, state, date, exercises }
    }
    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }
    #[cfg(feature = "ssr")]
    pub fn record(&self) -> Option<RecordId> {
        self.id.as_ref().map(|id|RecordId::from(("days",id)))
    }

    pub fn state(&self) -> DayState {
        self.state
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }
    pub fn exercises(&self) -> &Vec<Exercise> {
        &self.exercises
    }

}

