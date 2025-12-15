use crate::{backend::{
    domain::repositories::RoutineRepository,
    infrastructure::db::{establish_connection, DBPool},
}, entities::Routine, record_id};
use crate::backend::infrastructure::db::RoutineDB;
use std::sync::Arc;
use surrealdb::RecordId;
use crate::entities::DayTemplate;
use crate::utils::error::{AppError, AppRes};


#[derive(Clone)]
pub struct SurrealRoutineRepository {
    pool: DBPool,
}

impl SurrealRoutineRepository {
    pub async fn new() -> Self {
        Self {
            pool: establish_connection().await,
        }
    }
}
/*
    id: Option<String>,
    templates: Vec<DayTemplate>,
    weeks: [Week; 4],
    last_check_in: NaiveDate,
    last_day: NaiveDate,
    created_by: String,
    created_at: NaiveDate,*/
impl RoutineRepository for Arc<SurrealRoutineRepository> {
    async fn save(&self, device: String, routine: Routine) -> AppRes<()> {
        let routine: RoutineDB = routine.into();
        // let routine = RoutineDB::from(routine);
        let templates = routine.templates().into_iter().map(|DayTemplate(ex)|{
            ex.into_iter().map(|ex|ex.record()).collect::<Vec<Option<RecordId>>>()
        }).collect::<Vec<_>>();
        // let res = self.pool.insert(&routine).await;
        let res = self
            .pool
            .query(
                r#"
        insert into routines {
            device: $device,
            templates: $templates,
            week: $week,
            last_check_in: $last_check_in,
            last_day: $last_day,
            created_by: $created_by,
            created_at: $created_at,
        }
        "#,
            )
            .bind(("device",record_id!("device",device)))
            .bind(("templates", templates))
            .bind(("week", routine.weeks().to_owned()))
            .bind(("last_check_in",routine.last_check_in()))
            .bind(("last_day",routine.last_day()))
            .bind(("created_by",routine.created_by().to_owned()))
            .bind(("created_at",routine.created_at()))
            .await;
        match res {
            Ok(a) => {
                println!("{:#?}", a);
                Ok(())
            }
            Err(e) => Err(AppError::DBErr(56, e.to_string())),
        }
    }

    async fn delete(&self, id: &str) -> AppRes<()> {
        Err(AppError::IndexErr(1))
    }

    async fn get_all(&self) -> AppRes<Vec<Routine>> {
        Err(AppError::IndexErr(1))
    }

    async fn get_by_device(&self, id: &str) -> AppRes<Vec<Routine>> {
        Err(AppError::IndexErr(1))
    }

    async fn update(&self, persona: &Routine) -> AppRes<Routine> {
        Err(AppError::IndexErr(1))
    }
}