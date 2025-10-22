use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use surrealdb::RecordId;
#[cfg(feature = "ssr")]
use crate::backend::infrastructure::db::RoutineDB;
use crate::entities::{Day, Exercise};


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Routine {
    id: Option<String>,
    templates: Vec<DayTemplate>,
    weeks: [Week; 4],
    last_check_in: NaiveDate,
    last_day: NaiveDate,
    created_by: String,
    created_at: NaiveDate,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Week {
    pub completed: bool,
    pub days: [Day; 6],
}

impl Week {

}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DayTemplate(Vec<Exercise>);


impl Routine {
    pub fn new(id: Option<String>, templates: Vec<DayTemplate>, weeks: [Week; 4], last_check_in: NaiveDate, last_day: NaiveDate, created_by: String, created_at: NaiveDate) -> Self {
        Self { id, templates, weeks, last_check_in, last_day, created_by, created_at }
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }
    #[cfg(feature = "ssr")]
    pub fn record(&self) -> Option<RecordId> {
        self.id.as_ref().map(|id|RecordId::from(("routines",id)))
    }
    pub fn templates(&self) -> &Vec<DayTemplate> {
        &self.templates
    }

    pub fn weeks(&self) -> &[Week; 4] {
        &self.weeks
    }

    pub fn last_check_in(&self) -> NaiveDate {
        self.last_check_in
    }

    pub fn last_day(&self) -> NaiveDate {
        self.last_day
    }

    pub fn created_by(&self) -> &str {
        &self.created_by
    }

    pub fn created_at(&self) -> NaiveDate {
        self.created_at
    }

}

