use actix_web::{get, post, HttpResponse, Responder};
use actix_web::web::Json;
use crate::{backend::application::use_cases::exercise::SaveExerciseUseCase, string, entities::Exercise};
use crate::backend::infrastructure::repositories::SurrealExerciseRepository;
use crate::entities::{MuscleGroup, Series, ExerciseDTO};
use crate::utils::error::{AppError, AppRes};
use actix_web::web::Data;

#[post("/")]
pub async fn save_exercise(repo: Data<SurrealExerciseRepository>, exercise: Json<ExerciseDTO>) -> impl Responder {

    let exercise = exercise.into_inner();
    let (device, exercise) = exercise.into_inner();
    match SaveExerciseUseCase::new(repo.into_inner()).execute(device,exercise).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/")]
pub async fn get_exercises(repo: Data<SurrealExerciseRepository>) -> impl Responder {
    let ex = Exercise::new(None,string!("Curl"),[Some(Series::new(1,Some(1.4))),None,None,None],MuscleGroup::Biceps);
    HttpResponse::Ok().json(ex)
}