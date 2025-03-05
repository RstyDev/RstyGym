use crate::db::db;
use chrono::{Local, NaiveDate};
use sqlx::{FromRow, Pool, Sqlite, query, query_as};
use std::sync::Arc;
use sqlx::sqlite::SqliteQueryResult;
use structs::{
    day::{Day, DayState},
    day_template::DayTemplate,
    error::{AppError, AppRes as Res},
    exercise::{Exercise, Series},
    routine::Routine,
    week::Week,
};

pub struct App {
    name: String,
    routine: Option<Routine>,
    db: Arc<Pool<Sqlite>>,
}

impl App {
    pub async fn get() -> Res<App> {
        let db = Arc::new(db().await?);
        Ok(App {
            name: "Lucas".to_string(),
            routine: Routine::get(db.as_ref()).await?,
            db,
        })
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn routine(&self) -> Option<&Routine> {
        self.routine.as_ref()
    }
    pub async fn set_routine(&mut self, name: String, routine: Routine) -> Res<i64> {
        self.routine = Some(routine);
        self.routine
            .as_mut()
            .unwrap()
            .save(name, self.db.as_ref()).await
    }
    pub fn db(&self) -> &Pool<Sqlite> {
        self.db.as_ref()
    }
}

pub trait RoutineTrait: Sized {
    async fn from_db(routine: RoutineDB, db: &Pool<Sqlite>) -> Res<Self>;
    async fn get(db: &Pool<Sqlite>) -> Res<Option<Self>>;
    async fn save(&mut self, name: String, db: &Pool<Sqlite>) -> Res<i64>;
}
impl RoutineTrait for Routine {
    async fn from_db(routine: RoutineDB, db: &Pool<Sqlite>) -> Res<Self> {
        let temp_query = query_as!(
            DayTemplateDB,
            "select * from day_templates where routine = ?",
            routine.id
        )
        .fetch_all(db)
        .await
        .map_err(|e| AppError::DBErr(e.to_string()))?;
        let week_query = query_as!(WeekDB, "select * from weeks where routine = ?", routine.id)
            .fetch_all(db)
            .await
            .map_err(|e| AppError::DBErr(e.to_string()))?;
        let mut templates = Vec::new();
        for temp in temp_query {
            templates.push(DayTemplate::from_db(temp, db).await?);
        }
        let mut weeks = [
            Week::default(),
            Week::default(),
            Week::default(),
            Week::default(),
        ];
        for (i, week) in week_query.into_iter().enumerate() {
            weeks[i] = Week::from_db(week, db).await?
        }
        Ok(Self::build(
            Some(routine.id),
            templates,
            weeks,
            Some(routine.last_check_in),
            Some(routine.last_day_index as usize),
            routine.created_by,
            routine.created_at,
        ))
    }
    async fn get(db: &Pool<Sqlite>) -> Res<Option<Self>> {
        let routine_db = query_as!(
            RoutineDB,
            r#"select * from routines where id = (select max(id) from routines)"#
        )
        .fetch_optional(db)
        .await
        .map_err(|e| AppError::DBErr(e.to_string()))?;

        match routine_db {
            None => Ok(None),
            Some(a) => Ok(Some(Self::from_db(a, db).await?)),
        }
    }
    async fn save(&mut self, name: String, db: &Pool<Sqlite>) -> Res<i64> {
        let now = Local::now().date_naive();
        let date = NaiveDate::default();
        let res = query!(
            "insert into routines (last_check_in, last_day_index, created_by, created_at) values (?, ?, ?, ?)",
            date,
            -1,
            name,
            now
        ).execute(db).await.map_err(|e|AppError::DBErr(e.to_string()))?;
        let id = res.last_insert_rowid();
        self.set_id(id);
        for i in 0..self.templates().len() {
            self.template_at_mut(i)?.save(id, db).await?
        }
        for i in 0..4 {
            self.week_at_mut(i)?.save(id, db).await?
        }
        Ok(id)
    }
}
pub trait WeekTrait: Sized {
    async fn from_db(week: WeekDB, db: &Pool<Sqlite>) -> Res<Self>;
    async fn save(&mut self, id: i64, db: &Pool<Sqlite>) -> Res<()>;
}
impl WeekTrait for Week {
    async fn from_db(week: WeekDB, db: &Pool<Sqlite>) -> Res<Self> {
        let res = query_as!(DayDB, "select * from days where week = ?", week.id)
            .fetch_all(db)
            .await
            .map_err(|e| AppError::DBErr(e.to_string()))?;
        let mut days = [
            Day::default(),
            Day::default(),
            Day::default(),
            Day::default(),
            Day::default(),
            Day::default(),
        ];
        for (i, ser) in res.into_iter().enumerate() {
            days[i] = Day::from_db(ser, db).await?;
        }
        Ok(Self::build(Some(week.id), week.completed, days))
    }
    async fn save(&mut self, id: i64, db: &Pool<Sqlite>) -> Res<()> {
        let res = query!(
            "insert into weeks (completed, routine) values (?, ?)",
            *self.completed(),
            id
        )
        .execute(db)
        .await
        .map_err(|e| AppError::DBErr(e.to_string()))?;
        let id = res.last_insert_rowid();
        self.set_id(id);
        for i in 0..6 {
            self.day_at_mut(i)?.save(id, db).await?;
        }
        Ok(())
    }
}
pub trait DayTemplateTrait: Sized {
    async fn from_db(day: DayTemplateDB, db: &Pool<Sqlite>) -> Res<Self>;
    async fn save(&mut self, id: i64, db: &Pool<Sqlite>) -> Res<()>;
}
impl DayTemplateTrait for DayTemplate {
    async fn from_db(day: DayTemplateDB, db: &Pool<Sqlite>) -> Res<Self> {
        let res = query_as!(
            ExerciseDB,
            "select * from exercises where day_template = ?",
            day.id
        )
        .fetch_all(db)
        .await
        .map_err(|e| AppError::DBErr(e.to_string()))?;
        let mut exercises = Vec::new();
        for ser in res {
            exercises.push(Exercise::from_db(ser, db).await?);
        }
        Ok(Self::build(Some(day.id), exercises))
    }
    async fn save(&mut self, id: i64, db: &Pool<Sqlite>) -> Res<()> {
        let res = query!("insert into day_templates (routine) values (?)", id)
            .execute(db)
            .await
            .map_err(|e| AppError::DBErr(e.to_string()))?;
        let id = res.last_insert_rowid();
        self.set_id(id);
        for i in 0..self.exercises().len() {
            self.exercise_at_mut(i)?
                .save(DayOrTemplate::Template(id), db)
                .await?
        }
        Ok(())
    }
}
pub trait DayTrait: Sized {
    async fn from_db(day_db: DayDB, db: &Pool<Sqlite>) -> Res<Self>;
    async fn save(&mut self, id: i64, db: &Pool<Sqlite>) -> Res<()>;
}
impl DayTrait for Day {
    async fn from_db(day: DayDB, db: &Pool<Sqlite>) -> Res<Self> {
        let exs_db = query_as!(ExerciseDB, "select * from exercises where day = ?", day.id)
            .fetch_all(db)
            .await
            .map_err(|e| AppError::DBErr(e.to_string()))?;
        let mut exercises = Vec::new();
        for ex in exs_db {
            exercises.push(Exercise::from_db(ex, db).await?);
        }
        Ok(Day::build(
            Some(day.id),
            match day.state.as_str() {
                "Free" => DayState::Free,
                "Checked" => DayState::Checked,
                "Complete" => DayState::Complete,
                s => return Err(AppError::UnknownState(s.to_string())),
            },
            day.date,
            exercises,
        ))
    }
    async fn save(&mut self, id: i64, db: &Pool<Sqlite>) -> Res<()> {
        let state = self.state().to_string();
        let date = self.date().to_string();
        let res = query!(
            "insert into days (state, date, week) values (?, ?, ?)",
            state,
            date,
            id
        )
        .execute(db)
        .await
        .map_err(|e| AppError::DBErr(e.to_string()))?;
        let id = res.last_insert_rowid();
        self.set_id(id);
        for i in 0..self.exercises().len() {
            self.exercise_at_mut(i)?
                .save(DayOrTemplate::Day(id), db)
                .await?
        }
        Ok(())
    }
}
pub enum DayOrTemplate {
    Day(i64),
    Template(i64),
}
pub trait ExerciseTrait: Sized {
    async fn from_db(exercise: ExerciseDB, db: &Pool<Sqlite>) -> Res<Self>;
    async fn save(&mut self, id: DayOrTemplate, db: &Pool<Sqlite>) -> Res<()>;
}
impl ExerciseTrait for Exercise {
    async fn from_db(exercise: ExerciseDB, db: &Pool<Sqlite>) -> Res<Self> {
        let res = query_as!(
            SeriesDB,
            r#"select id, exercise, count as "count:_", weight as "weight:_"  from series where exercise = ?"#,
            exercise.id
        ).fetch_all(db).await.map_err(|e|AppError::DBErr(e.to_string()))?;
        let mut series = [None, None, None, None];
        for (i, ser) in res.into_iter().enumerate() {
            series[i] = Some(Series::from_db(ser).await);
        }

        Ok(Exercise::build(
            Some(exercise.id),
            exercise.name,
            series,
            exercise.muscle_group.try_into().map_err(|a| AppError::IndexErr)?,
        ))
    }

    async fn save(&mut self, id: DayOrTemplate, db: &Pool<Sqlite>) -> Res<()> {
        let name = self.name();
        let group = self.group().to_string();
        let result = match id {
            DayOrTemplate::Day(d) => query!("insert into exercises (name, muscle_group, day) values (?, ?, ?)",name,group,d).execute(db).await.map_err(|e| AppError::DBErr(e.to_string())),
            DayOrTemplate::Template(d) => query!("insert into exercises (name, muscle_group, day) values (?, ?, ?)",name,group,d).execute(db).await.map_err(|e| AppError::DBErr(e.to_string())),
        };
        let res = match result {
            Ok(a) => a,
            Err(e) => { println!("{e}");return Err(e) }
        };
        // let (string, id) = match id {
        //     DayOrTemplate::Day(d) => ("day", d),
        //     DayOrTemplate::Template(d) => ("day_template", d),
        // };
        // let query_str = format!(
        //     "insert into exercises (name, group, {}) values (?, ?, ?)",
        //     string
        // );
        // println!("{}",query_str);
        // let res = query(
        //     query_str
        //     .as_str(),
        // )
        // .bind(self.name())
        // .bind(self.group().to_string())
        // .bind(id)
        // .execute(db)
        // .await
        // .map_err(|e| AppError::DBErr(format!("{e}\nQuery: {query_str}")))?;
        let id = res.last_insert_rowid();
        self.set_id(id);
        for i in 0..4 {
            if let Some(series) = self.series_at_mut(i) {
                series.save(id, db).await?
            }
        }
        Ok(())
    }
}
pub trait SeriesTrait {
    async fn from_db(series_db: SeriesDB) -> Self;
    async fn save(&mut self, id: i64, db: &Pool<Sqlite>) -> Res<()>;
}
impl SeriesTrait for Series {
    async fn from_db(series: SeriesDB) -> Self {
        Self::build(Some(series.id), series.count, series.weight)
    }

    async fn save(&mut self, id: i64, db: &Pool<Sqlite>) -> Res<()> {
        let weight = self.weight().map(|f|*f);
        let res = query!(
            "insert into series (exercise, count, weight) values (?, ?, ?)",
            id,
            *self.count(),
            weight
        )
        .execute(db)
        .await
        .map_err(|e| AppError::DBErr(e.to_string()))?;
        self.set_id(res.last_insert_rowid());
        Ok(())
    }
}

#[derive(FromRow)]
pub struct BigIntDB {
    pub int: i64,
}
#[derive(FromRow)]
pub struct IntDB {
    pub int: i32,
}
#[derive(FromRow)]
pub struct DoubleDB {
    pub double: f64,
}
#[derive(FromRow)]
pub struct FloatDB {
    pub float: f32,
}
#[derive(FromRow)]
pub struct BoolDB {
    pub val: bool,
}
#[derive(FromRow)]
pub struct StringDB {
    pub string: String,
}
#[derive(FromRow)]
pub struct DayDB {
    pub id: i64,
    pub state: String,
    pub date: NaiveDate,
    pub week: i64,
}
#[derive(FromRow)]
pub struct WeekDB {
    pub id: i64,
    pub completed: bool,
    pub routine: i64,
}
#[derive(FromRow)]
pub struct RoutineDB {
    pub id: i64,
    pub last_check_in: NaiveDate,
    pub last_day_index: i64,
    pub created_by: Option<String>,
    pub created_at: NaiveDate,
}
#[derive(FromRow)]
pub struct SeriesDB {
    pub id: i64,
    pub exercise: i64,
    pub count: u8,
    pub weight: Option<f32>,
}
#[derive(FromRow)]
pub struct ExerciseDB {
    pub id: i64,
    pub muscle_group: String,
    pub name: String,
    pub day: Option<i64>,
    pub day_template: Option<i64>,
}
#[derive(FromRow)]
pub struct DayTemplateDB {
    id: i64,
    routine: i64,
}