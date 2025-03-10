use crate::{
    day_template::DayTemplate,
    error::{AppError, AppRes as Res},
    week::Week,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::day::Day;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Routine {
    id: i64,
    templates: Vec<DayTemplate>,
    weeks: [Week; 4],
    last_check_in: NaiveDate,
    last_day_index: Option<usize>,
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
            last_day_index: last_day_index,
            created_by: created_by.unwrap_or_default(),
            created_at,
        }
    }
    pub fn id(&self) -> &i64 {
        &self.id
    }
    pub fn set_id(&mut self, id: i64) {
        self.id = id;
    }
    pub fn templates(&self) -> &Vec<DayTemplate> {
        &self.templates
    }
    pub fn template_at(&self, index: usize) -> Res<&DayTemplate> {
        if &self.templates.len() > &index {
            Ok(&self.templates[index])
        } else {
            Err(AppError::IndexErr(53))
        }
    }
    pub fn template_at_mut(&mut self, index: usize) -> Res<&mut DayTemplate> {
        if &self.templates.len() > &index {
            Ok(&mut self.templates[index])
        } else {
            Err(AppError::IndexErr(60))
        }
    }
    pub fn weeks(&self) -> &[Week; 4] {
        &self.weeks
    }
    pub fn week_at(&self, index: usize) -> Res<&Week> {
        if &self.weeks.len() > &index {
            Ok(&self.weeks[index])
        } else {
            Err(AppError::IndexErr(70))
        }
    }
    pub fn today(&self) -> Option<&Day> {
        self.weeks().into_iter().find_map(|w|w.today())
    }
    pub fn this_week_mut(&mut self) -> Option<&mut Week> {
        let mut res = None;
        let len = self.weeks.len();

        for i in 0..len {
            if self.weeks[i].is_current(){
                res = Some(&mut self.weeks[i]);
                break;
            }
        }
        res
    }
    pub fn set_weeks(&mut self, weeks: [Week; 4]) {
        self.weeks = weeks;
    }
    pub fn set_week_at(&mut self, week: Week, index: usize) -> Res<()> {
        if &self.weeks.len() > &index {
            self.weeks[index] = week;
            Ok(())
        } else {
            Err(AppError::IndexErr(96))
        }
    }
    pub fn week_at_mut(&mut self, index: usize) -> Res<&mut Week> {
        if &self.weeks.len() > &index {
            Ok(&mut self.weeks[index])
        } else {
            Err(AppError::IndexErr(103))
        }
    }
    pub fn is_current(&self) -> bool {
        self.weeks.iter().any(|w| w.is_current())
    }

    pub fn last_check_in(&self) -> NaiveDate {
        self.last_check_in
    }
    pub fn set_last_check_in(&mut self, date: NaiveDate) {
        self.last_check_in = date;
    }
    pub fn last_day_index(&self) -> Option<usize> {
        self.last_day_index
    }
    pub fn set_last_day_index(&mut self, index: usize) {
        self.last_day_index = Some(index);
    }
}

impl PartialEq for Routine {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
