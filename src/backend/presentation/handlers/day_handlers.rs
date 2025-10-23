use actix_web::{post, get, put, HttpResponse, Responder};
use actix_web::web::{Data, Json, Path};
use chrono::Local;
use crate::{entities::Day, backend::{application::use_cases::day::*, infrastructure::repositories::SurrealDayRepository}};
use crate::entities::{DayDTO, DayState, Exercise};

#[post("/")]
pub async fn save_day(repo: Data<SurrealDayRepository>, day: Json<DayDTO>) -> impl Responder {

    let day = day.into_inner();
    let (device,day) = day.into_inner();
    match SaveDayUseCase::new(repo.into_inner()).execute(device,day).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
#[get("/")]
pub async fn get_days(repo: Data<SurrealDayRepository>) -> impl Responder {
    let day = Day::new(None,DayState::Free,Local::now().date_naive(),vec![]);
    HttpResponse::Ok().json(day)
}

#[put("/{id}")]
pub async fn update_exercises(repo: Data<SurrealDayRepository>, id: Path<String>, Json(exercises): Json<Vec<Exercise>>) -> impl Responder {
    match UpdateExercisesUseCase::new(repo.into_inner()).execute(id.into_inner(), exercises).await {
        Ok(day) => HttpResponse::Ok().json(day),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}