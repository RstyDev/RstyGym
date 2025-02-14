use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::{day_template::DayTemplate, error::AppRes as Res, week::Week};
use crate::error::AppError;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Routine {
    id: i64,
    templates: Vec<DayTemplate>,
    weeks: [Week;4],
    last_check_in: NaiveDate
}
impl Routine {
    pub fn build(id: Option<i64>, templates: Vec<DayTemplate>, weeks: [Week;4]) -> Routine {
        Routine {
            id: id.unwrap_or_default(),
            templates,
            weeks,
        }
    }
    pub fn templates(&self) -> &Vec<DayTemplate> {
        &self.templates
    }
    pub fn template_at(&self, index: usize) -> Res<&DayTemplate> {
        if self.templates.len() > &index {
            Ok(&self.templates[index])
        }else{
            Err(AppError::IndexErr)
        }

    }
    pub fn weeks(&self) -> &[Week;4] {
        &self.weeks
    }
    pub fn week_at(&self, index: usize) -> Res<&Week> {
        if self.weeks.len() > &index {
            Ok(&self.weeks[index])
        }else{
            Err(AppError::IndexErr)
        }
    }
}