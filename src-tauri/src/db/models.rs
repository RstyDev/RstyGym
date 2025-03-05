use crate::db::db;
use chrono::{Local, NaiveDate};
use sqlx::{FromRow, Pool, Sqlite, query_as, query};
use std::sync::Arc;
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
    pub fn set_routine(&mut self, routine: Routine) -> Res<i64> {
        self.routine = Some(routine);
        self.routine.as_mut().unwrap().save(self.db.as_ref())?
    }
    pub fn db(&self) -> &Pool<Sqlite> {
        self.db.as_ref()
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
    pub group: String,
    pub name: String,
    pub day: Option<i64>,
    pub day_template: Option<i64>,
}
#[derive(FromRow)]
pub struct DayTemplateDB {
    id: i64,
    routine: i64,
}
pub trait RoutineTrait: Sized {
    async fn from_db(routine: RoutineDB, db: &Pool<Sqlite>) -> Res<Self>;
    async fn get(db: &Pool<Sqlite>) -> Res<Option<Self>>;
    async fn save(&mut self, db: &Pool<Sqlite>) -> Res<()>;
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
/*
'id' INTEGER NOT NULL,
    'last_check_in' DATE NOT NULL,
    'last_day_index' INTEGER NOT NULL,
    'created_by' TEXT,
    'created_at' DATE NOT NULL,*/
    async fn save(&mut self, db: &Pool<Sqlite>) -> Res<()> {
        let res = query("insert into routines (last_check_in, last_day_index, created_by, created_at) values (?, ?, ?, ?)").bind(NaiveDate::default()).bind(-1).bind("Lucas").bind(Local::now().date_naive()).execute(db).await.map_err(|e|AppError::DBErr(e.to_string()))?;
        self.set_id(res.last_insert_rowid());
        for i in 0..self.templates().len(){
            self.template_at_mut(i)?.save(*self.id(),db)?
        }
        for i in 0..4{
            self.week_at_mut(i)?.save(*self.id(),db)?
        }
        //TODO!!
        Ok(())
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
        todo!()
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
    async fn save(&mut self,id: i64, db: &Pool<Sqlite>) -> Res<()> {
        let res = query("insert into day_templates (routine) values (?)").bind(id).execute(db).await.map_err(|e|AppError::DBErr(e.to_string()))?;
        self.set_id(res.last_insert_rowid());
        for i in 0..self.exercises().len(){
            self.exercise_at_mut(i)?.save(DayOrTemplate::Template(id),db)?
        }
        Ok(())
    }
}
pub trait DayTrait: Sized {
    async fn from_db(day_db: DayDB, db: &Pool<Sqlite>) -> Res<Self>;
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
    /*
    'id' INTEGER NOT NULL,
    'exercise' INTEGER NOT NULL,
    'count' INTEGER NOT NULL,
    'weight' REAL,
    */
    async fn from_db(exercise: ExerciseDB, db: &Pool<Sqlite>) -> Res<Self> {
        let res = query_as!(SeriesDB,r#"select id, exercise, count as "count:_", weight as "weight:_"  from series where exercise = ?"#,exercise.id).fetch_all(db).await.map_err(|e|AppError::DBErr(e.to_string()))?;
        let mut series = [None, None, None, None];
        for (i, ser) in res.into_iter().enumerate() {
            series[i] = Some(Series::from_db(ser).await);
        }

        Ok(Exercise::build(
            Some(exercise.id),
            exercise.name,
            series,
            exercise.group.try_into().map_err(|a| AppError::IndexErr)?,
        ))
    }
/*
'name' TEXT NOT NULL,
    'group' TEXT NOT NULL,
    'day' INTEGER,
    'day_template' INTEGER,*/
    async fn save(&mut self, id: DayOrTemplate, db: &Pool<Sqlite>) -> Res<()> {
        let (string,id) = match id{
            DayOrTemplate::Day(d) => ("day", d),
            DayOrTemplate::Template(d) => ("day_template", d),
        };
        let res = query(format!("insert into exercises (name, group, {}) values (?, ?, ?)",string).as_str()).bind(self.name()).bind(self.group().to_string()).bind(id).execute(db).await.map_err(|e|AppError::DBErr(e.to_string()))?;
        self.set_id(res.last_insert_rowid());
        for i in 0..4{
            if let Some(series) = self.series_at_mut(i) {
                series.save(*self.id(), db)?
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
/*
'exercise' INTEGER NOT NULL,
    'count' INTEGER NOT NULL,
    'weight' REAL,*/
    async fn save(&mut self, id: i64, db: &Pool<Sqlite>) -> Res<()> {
        let res = query("insert into series (exercise, count, weight) values (?, ?, ?)").bind(id).bind(*self.count()).bind(self.weight()).execute(db).await.map_err(|e|AppError::DBErr(e.to_string()))?;
        self.set_id(res.last_insert_rowid());
        Ok(())
    }
}
