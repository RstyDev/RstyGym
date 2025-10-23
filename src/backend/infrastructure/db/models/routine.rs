use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use crate::backend::infrastructure::db::DayDB;
use crate::{day_db, record_id};
use crate::entities::{Day, DayTemplate, Exercise, Routine, Week};


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RoutineDB {
    id: Option<RecordId>,
    templates: Vec<DayTemplate>,
    weeks: [WeekDB; 4],
    last_check_in: NaiveDate,
    last_day: NaiveDate,
    created_by: String,
    created_at: NaiveDate,
}

impl From<Routine> for RoutineDB {
    fn from(value: Routine) -> Self {
        RoutineDB::new(value.id().cloned(),value.templates().to_owned(),value.weeks().to_owned(),value.last_check_in().to_owned(),value.last_day().to_owned(),value.created_by().to_owned(),value.created_at().to_owned())
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WeekDB {
    pub completed: bool,
    pub days: [DayDB; 6],
}

impl From<Week> for WeekDB {
    fn from(value: Week) -> Self {
        WeekDB { completed: value.completed, days: [
            day_db!(value.days[0].to_owned()),
            day_db!(value.days[1].to_owned()),
            day_db!(value.days[2].to_owned()),
            day_db!(value.days[3].to_owned()),
            day_db!(value.days[4].to_owned()),
            day_db!(value.days[5].to_owned())
        ]}
    }
}
impl WeekDB {
    pub fn build(week: Week) -> Self {
        let days = week.days;
        let days = [days[0].to_owned().into(),days[1].to_owned().into(),days[2].to_owned().into(),days[3].to_owned().into(),days[4].to_owned().into(),days[5].to_owned().into()];
        Self { completed: week.completed, days }
    }
}



impl RoutineDB {
    pub fn new(id: Option<String>, templates: Vec<DayTemplate>, weeks: [Week; 4], last_check_in: NaiveDate, last_day: NaiveDate, created_by: String, created_at: NaiveDate) -> Self {
        let id = id.map(|id|record_id!("routines",id));
        let weeks = [weeks[0].to_owned().into(),weeks[1].to_owned().into(),weeks[2].to_owned().into(),weeks[3].to_owned().into()];
        Self { id, templates, weeks, last_check_in, last_day, created_by, created_at }
    }

    pub fn id(&self) -> Option<&RecordId> {
        self.id.as_ref()
    }

    pub fn templates(&self) -> &Vec<DayTemplate> {
        &self.templates
    }

    pub fn weeks(&self) -> &[WeekDB; 4] {
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