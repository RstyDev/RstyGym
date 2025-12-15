use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use surrealdb::RecordId;
use crate::entities::{Day, DayState, Exercise};
use crate::record_id;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DayDB {
    id: Option<RecordId>,
    state: DayState,
    date: NaiveDate,
    exercises: Vec<RecordId>,
}

impl From<Day> for DayDB {
    fn from(value: Day) -> Self {
        DayDB::new(value.id().cloned(), value.state(), value.date(), value.exercises().to_owned())
    }
}


impl DayDB {
    pub fn new(id: Option<String>, state: DayState, date: NaiveDate, exercises: Vec<Exercise>) -> Self {
        let id = id.map(|id|record_id!("days",id));
        Self { id, state, date, exercises: exercises.into_iter().map(|ex|ex.record().unwrap()).collect() }
    }
    pub fn build(day: Day) -> Self {
        Self {
            id: day.record(),
            state: day.state(),
            date: day.date(),
            exercises: day.exercises().into_iter().map(|ex|ex.record().unwrap()).collect::<Vec<_>>(),
        }
    }
    pub fn id(&self) -> Option<&RecordId> {
        self.id.as_ref()
    }

    pub fn state(&self) -> DayState {
        self.state
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }
    pub fn exercises(&self) -> &Vec<RecordId> {
        &self.exercises
    }

}