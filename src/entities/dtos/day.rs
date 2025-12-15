use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::entities::{Day, DayState, Exercise};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DayDTO {
    pub device: String,
    pub id: Option<String>,
    pub state: DayState,
    pub date: NaiveDate,
    pub exercises: Vec<Exercise>,
}

impl DayDTO {
    pub fn into_inner(self) -> (String, Day) {
        (self.device,Day::new(self.id,self.state,self.date,self.exercises))
    }
}