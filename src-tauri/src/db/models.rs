use chrono::NaiveDate;
use sqlx::{query_as, FromRow, Pool, Sqlite};
use structs::{
    day::{Day, DayState},
    error::{AppRes as Res, AppError},
    exercise::{Exercise, Series}
};
use structs::day_template::DayTemplate;
use structs::routine::Routine;
use structs::week::Week;

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
    pub last_day_index: usize,
    pub created_by: String,
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
    pub day_template: Option<i64>
}
#[derive(FromRow)]
pub struct DayTemplateDB {
    id: i64,
    routine: i64,
}
pub trait RoutineTrait: Sized {
    async fn from_db(routine: RoutineDB, db: &Pool<Sqlite>) -> Res<Self>;
}
impl RoutineTrait for Routine {
    async fn from_db(routine: RoutineDB, db: &Pool<Sqlite>) -> Res<Self> {
        let temp_query = query_as!(DayTemplateDB,"select * from day_templates where routine = ?",routine.id).fetch_all(db).await.map_err(|e|AppError::DBErr(e.to_string()))?;
        let week_query = query_as!(WeekDB,"select * from weeks where routine = ?",routine.id).fetch_all(db).await.map_err(|e|AppError::DBErr(e.to_string()))?;
        let mut templates = Vec::new();
        for temp in temp_query{
            templates.push(DayTemplate::from_db(temp,db).await?);
        }
        let mut weeks = [Week::default(),Week::default(),Week::default(),Week::default()];
        for (i,week) in week_query.into_iter().enumerate() {
            weeks[i] = Week::from_db(week,db).await?
        }
        Ok(Self::build(Some(routine.id),templates,weeks,Some(routine.last_check_in),Some(routine.last_day_index),Some(routine.created_by),routine.created_at))
    }
}
pub trait WeekTrait: Sized {
    async fn from_db(week: WeekDB, db: &Pool<Sqlite>) -> Res<Self>;
}
impl WeekTrait for Week {
    async fn from_db(week: WeekDB, db: &Pool<Sqlite>) -> Res<Self> {
        let res = query_as!(DayDB, "select * from days where week = ?",week.id).fetch_all(db).await.map_err(|e|AppError::DBErr(e.to_string()))?;
        let mut days = [Day::default(),Day::default(),Day::default(),Day::default(),Day::default(),Day::default()];
        for (i,ser) in res.into_iter().enumerate() {
            days[i] = Day::from_db(ser, db).await?;
        }
        Ok(Self::build(Some(week.id), week.completed, days))
    }
}
pub trait DayTemplateTrait: Sized {
    async fn from_db(day: DayTemplateDB, db: &Pool<Sqlite>) -> Res<Self>;
}
impl DayTemplateTrait for DayTemplate {
    async fn from_db(day: DayTemplateDB, db: &Pool<Sqlite>) -> Res<Self> {
        let res = query_as!(ExerciseDB,"select * from exercises where day_template = ?",day.id).fetch_all(db).await.map_err(|e|AppError::DBErr(e.to_string()))?;
        let mut exercises = Vec::new();
        for ser in res{
            exercises.push(Exercise::from_db(ser,db).await?);
        }
        Ok(Self::build(Some(day.id), exercises))
    }
}
pub trait DayTrait: Sized {
    async fn from_db(day_db: DayDB, db: &Pool<Sqlite>) -> Res<Self>;
}
impl DayTrait for Day {
    async fn from_db(day: DayDB, db: &Pool<Sqlite>) -> Res<Self> {
        let exs_db = query_as!(ExerciseDB,"select * from exercises where day = ?",day.id).fetch_all(db).await.map_err(|e|AppError::DBErr(e.to_string()))?;
        let mut exercises= Vec::new();
        for ex in exs_db {
            exercises.push(Exercise::from_db(ex, db).await?);
        }
        Ok(Day::build(Some(day.id), match day.state.as_str() {
            "Free" => DayState::Free,
            "Checked" => DayState::Checked,
            "Complete" => DayState::Complete,
            s => return Err(AppError::UnknownState(s.to_string()))
        }, day.date,exercises))
    }
}
pub trait ExerciseTrait: Sized {
    async fn from_db(exercise: ExerciseDB, db: &Pool<Sqlite>) -> Res<Self>;
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
        let mut series = [None,None,None,None];
        for (i,ser) in res.into_iter().enumerate() {
            series[i] = Some(Series::from_db(ser).await);
        }

        Ok(Exercise::build(Some(exercise.id),exercise.name.as_str(),series,exercise.group.try_into().map_err(|a|{AppError::IndexErr})?))
    }
}
pub trait SeriesTrait {
    async fn from_db(series_db: SeriesDB) -> Self;
}
impl SeriesTrait for Series {
    async fn from_db(series: SeriesDB) -> Self {
        Self::build(Some(series.id),series.count,series.weight)
    }
}