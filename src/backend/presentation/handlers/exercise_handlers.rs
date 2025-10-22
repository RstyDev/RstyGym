use actix_web::{get, post, HttpResponse, Responder};
use actix_web::web::Json;
use crate::backend::application::use_cases::exercise::SaveExerciseUseCase;
use crate::backend::infrastructure::repositories::SurrealExerciseRepository;
use crate::entities::{Exercise, MuscleGroup, Series};
use crate::error::{AppError, AppRes};
use actix_web::web::Data;

#[post("/")]
pub async fn save_exercise(repo: Data<SurrealExerciseRepository>, exercise: Json<Exercise>) -> impl Responder {

    let exercise = exercise.into_inner();
    match SaveExerciseUseCase::new(repo.into_inner()).execute(exercise).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/")]
pub async fn get_exercises(repo: Data<SurrealExerciseRepository>) -> impl Responder {
    let ex = Exercise::new(None,String::from("Curl"),[Some(Series::new(1,Some(1.4))),None,None,None],MuscleGroup::Biceps);
    HttpResponse::Ok().json(ex)
}