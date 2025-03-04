use crate::{
    day_template::DayTemplate,
    error::{AppError, AppRes as Res},
    week::Week,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Routine {
    id: i64,
    templates: Vec<DayTemplate>,
    weeks: [Week; 4],
    last_check_in: NaiveDate,
    last_day_index: usize,
    created_by: String,
    created_at: NaiveDate,
}
impl Routine {
    pub fn build(
        id: Option<i64>,
        templates: Vec<DayTemplate>,
        weeks: [Week; 4],
        last_check_in: Option<NaiveDate>,
        last_day_index: Option<usize>,
        created_by: Option<String>,
        created_at: NaiveDate,
    ) -> Routine {
        Routine {
            id: id.unwrap_or_default(),
            templates,
            weeks,
            last_check_in: last_check_in.unwrap_or_default(),
            last_day_index: last_day_index.unwrap_or_default(),
            created_by: created_by.unwrap_or_default(),
            created_at,
        }
    }
    pub fn templates(&self) -> &Vec<DayTemplate> {
        &self.templates
    }
    pub fn template_at(&self, index: usize) -> Res<&DayTemplate> {
        if &self.templates.len() > &index {
            Ok(&self.templates[index])
        } else {
            Err(AppError::IndexErr)
        }
    }
    pub fn weeks(&self) -> &[Week; 4] {
        &self.weeks
    }
    pub fn week_at(&self, index: usize) -> Res<&Week> {
        if &self.weeks.len() > &index {
            Ok(&self.weeks[index])
        } else {
            Err(AppError::IndexErr)
        }
    }
    pub fn set_weeks(&mut self, weeks: [Week; 4]) {
        self.weeks = weeks;
    }
    pub fn set_week_at(&mut self, week: Week, index: usize) -> Res<()> {
        if &self.weeks.len() > &index {
            self.weeks[index] = week;
            Ok(())
        } else {
            Err(AppError::IndexErr)
        }
    }
    pub fn week_at_mut(&mut self, index: usize) -> Res<&mut Week> {
        if &self.weeks.len() > &index {
            Ok(&mut self.weeks[index])
        } else {
            Err(AppError::IndexErr)
        }
    }
    pub fn is_current(&self) -> bool {
        self.weeks.iter().any(|w| w.is_current())
    }
}

impl PartialEq for Routine {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
