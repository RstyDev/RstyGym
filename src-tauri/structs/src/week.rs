use serde::{Deserialize, Serialize};
use crate::{day::Day, error::AppRes as Res};
use crate::error::AppError;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Week {
    id: i64,
    completed: bool,
    days: [Day;6],
}
impl Week {
    pub fn build(id: Option<i64>, completed: bool, days: [Day;6]) -> Week {
        Week{
            id: id.unwrap_or_default(),
            completed,
            days,
        }
    }
    pub fn id(&self) -> &i64 {
        &self.id
    }
    pub fn set_id(&mut self, id: i64) {
        self.id = id;
    }
    pub fn completed(&self) -> &bool {
        &self.completed
    }
    pub fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
    }
    pub fn days(&self) -> &[Day;6]{
        &self.days
    }
    pub fn day_at(&self, index: usize) -> Res<&Day> {
        if &self.days.len() > &index {
            Ok(&self.days[index])
        }else{
            Err(AppError::IndexErr)
        }
    }
    pub fn set_days(&mut self, days: [Day;6]) {
        self.days = days;
    }
    pub fn set_day_at(&mut self, day: Day, index: usize) -> Res<()> {
        if &self.days.len() > &index {
            self.days[index] = day;
            Ok(())
        }else {Err(AppError::IndexErr)}
    }
    pub fn day_at_mut(&mut self, index: usize) -> Res<&mut Day> {
        if &self.days.len() > &index {
            Ok(&mut self.days[index])
        }else{
            Err(AppError::IndexErr)
        }

    }
}