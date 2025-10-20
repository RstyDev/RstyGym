use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
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
    completed: bool,
    days: [Day; 6],
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DayTemplate(Vec<Exercise>);


