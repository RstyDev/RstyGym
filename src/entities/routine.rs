use chrono::{Datelike, Days, Local, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use surrealdb::RecordId;
#[cfg(feature = "ssr")]
use crate::backend::infrastructure::db::RoutineDB;
use crate::entities::{Day, DayState, Exercise};



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
    pub fn from_day(day: NaiveDate) -> Self {
        let days = [
            Day::new(None,DayState::Free,day,vec![]),
            Day::new(None,DayState::Free,day,vec![]),
            Day::new(None,DayState::Free,day,vec![]),
            Day::new(None,DayState::Free,day,vec![]),
            Day::new(None,DayState::Free,day,vec![]),
            Day::new(None,DayState::Free,day,vec![]),
        ];
        Self { completed: false, days }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DayTemplate(pub Vec<Exercise>);


impl Routine {
    pub fn new(id: Option<String>, templates: Vec<DayTemplate>, weeks: [Week; 4], last_check_in: NaiveDate, last_day: NaiveDate, created_by: String, created_at: NaiveDate) -> Self {
        Self { id, templates, weeks, last_check_in, last_day, created_by, created_at }
    }
    pub fn from_templates(created_by: String, templates: Vec<DayTemplate>) -> Self {
        let today = Local::now().date_naive();
        let minus_days = match today.weekday() {
            Weekday::Mon => 0,
            Weekday::Tue => 1,
            Weekday::Wed => 2,
            Weekday::Thu => 3,
            Weekday::Fri => 4,
            Weekday::Sat => 5,
            Weekday::Sun => 6,
        };
        let last_monday = today.checked_sub_days(Days::new(minus_days)).unwrap();

        Self {
            id: None,
            templates,
            weeks: [
                Week::from_day(last_monday),
                Week::from_day(last_monday.checked_add_days(Days::new(7)).unwrap()),
                Week::from_day(last_monday.checked_add_days(Days::new(14)).unwrap()),
                Week::from_day(last_monday.checked_add_days(Days::new(21)).unwrap()),
            ],
            last_check_in: NaiveDate::MIN,
            last_day: Default::default(),
            created_by,
            created_at: today,
        }
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

