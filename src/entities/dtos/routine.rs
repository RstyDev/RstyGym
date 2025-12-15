use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::entities::{DayTemplate, Routine, Week};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct NewRoutineDTO {
    pub device: String,
    pub created_by: String,
    pub templates: Vec<DayTemplate>
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct RoutineDTO {
    pub device: String,
    pub id: Option<String>,
    pub templates: Vec<DayTemplate>,
    pub weeks: [Week; 4],
    pub last_check_in: NaiveDate,
    pub last_day: NaiveDate,
    pub created_by: String,
    pub created_at: NaiveDate,
}

impl RoutineDTO {
    pub fn into_inner(self) ->(String,Routine) {
        (self.device,Routine::new(self.id,self.templates,self.weeks,self.last_check_in,self.last_day,self.created_by,self.created_at))
    }
}