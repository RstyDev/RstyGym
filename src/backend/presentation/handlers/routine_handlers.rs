use actix_web::{post, get, HttpResponse, Responder};
use actix_web::web::{Data, Json};
use chrono::Local;
use crate::{string,entities::Day, backend::{application::use_cases::routine::SaveRoutineUseCase, infrastructure::repositories::SurrealRoutineRepository}};
use crate::entities::{DayTemplate, Exercise, MuscleGroup, NewRoutineDTO, Routine, Week};

#[post("/")]
pub async fn save_routine(repo: Data<SurrealRoutineRepository>, days: Json<NewRoutineDTO>) -> impl Responder {

    let days = days.into_inner();

    let NewRoutineDTO {
        device,
        created_by,
        templates
    } = days;
    let routine = Routine::from_templates(created_by,templates);
    match SaveRoutineUseCase::new(repo.into_inner()).execute(device,routine).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
#[get("/")]
pub async fn get_routines(repo: Data<SurrealRoutineRepository>) -> impl Responder {
    let routine = Routine::new(None,vec![DayTemplate(vec![Exercise::new(None,string!("Chest Press"),[None,None,None,None],MuscleGroup::Chest)])],[Week::default(),Week::default(),Week::default(),Week::default()],Local::now().date_naive(),Local::now().date_naive(),string!("Lucas"),Local::now().date_naive());
    HttpResponse::Ok().json(routine)
}