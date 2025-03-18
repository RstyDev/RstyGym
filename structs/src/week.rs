use crate::day::DayState;
use crate::{
    day::Day,
    error::{AppError, AppRes as Res},
};
use chrono::{Datelike, Days, Local, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Week {
    id: i64,
    completed: bool,
    days: [Day; 6],
}
impl Week {
    pub fn build(id: Option<i64>, completed: bool, days: [Day; 6]) -> Week {
        Week {
            id: id.unwrap_or_default(),
            completed,
            days,
        }
    }
    pub fn build_from_day(day: NaiveDate) -> Week {
        let mut days = [
            Day::default(),
            Day::default(),
            Day::default(),
            Day::default(),
            Day::default(),
            Day::default(),
        ];
        let mut past = false;
        for i in 0..days.len() {
            let day = day.checked_add_days(Days::new(i as u64)).unwrap();
            if day.weekday() == Weekday::Sun {
                past = true;
            }

            if past {
                days[i] = Day::build(
                    None,
                    DayState::Free,
                    day.checked_add_days(Days::new(1)).unwrap(),
                    vec![],
                );
            } else {
                days[i] = Day::build(None, DayState::Free, day, vec![]);
            }
        }
        Week {
            id: i64::default(),
            completed: false,
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
    pub fn days(&self) -> &[Day; 6] {
        &self.days
    }
    pub fn day_at(&self, index: usize) -> Res<&Day> {
        if &self.days.len() > &index {
            Ok(&self.days[index])
        } else {
            Err(AppError::IndexErr(70))
        }
    }
    pub fn today(&self) -> Option<&Day> {
        self.days()
            .into_iter()
            .find(|d| d.date() == &Local::now().date_naive())
    }
    pub fn today_mut(&mut self) -> Option<&mut Day> {
        let mut res = None;
        for i in 0..self.days.len() {
            if self.days[i].date() == &Local::now().date_naive() {
                res = Some(&mut self.days[i]);
                break;
            }
        }
        res
    }
    pub fn set_today(&mut self, day: Day) {
        let i = self
            .days()
            .into_iter()
            .enumerate()
            .find_map(|d| (d.1.date() == &Local::now().date_naive()).then_some(d.0))
            .unwrap();
        self.days[i] = day;
    }
    pub fn last_same_as(&self, day: &Day) -> Option<&Day> {
        self.days().into_iter().rev().find(|&d|d.exercises() == day.exercises() && d.date() != day.date() )
    }
    pub fn set_days(&mut self, days: [Day; 6]) {
        self.days = days;
    }
    pub fn set_day_at(&mut self, day: Day, index: usize) -> Res<()> {
        if &self.days.len() > &index {
            self.days[index] = day;
            Ok(())
        } else {
            Err(AppError::IndexErr(94))
        }
    }
    pub fn day_at_mut(&mut self, index: usize) -> Res<&mut Day> {
        if &self.days.len() > &index {
            Ok(&mut self.days[index])
        } else {
            Err(AppError::IndexErr(101))
        }
    }

    pub fn is_current(&self) -> bool {
        self.days[0].date() <= &Local::now().date_naive()
            && self.days[5].date() >= &Local::now().date_naive()
    }
}
