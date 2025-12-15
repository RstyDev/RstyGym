use crate::{backend::infrastructure::db::establish_connection, string};
use crate::backend::infrastructure::repositories::{SurrealRoutineRepository, SurrealExerciseRepository, SurrealDayRepository};
use crate::backend::{
    presentation::routes::root_routes,
};
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;


pub async fn run() -> std::io::Result<()> {
    dotenv().ok();
    let routine_repo = SurrealRoutineRepository::new().await;
    let day_repo = SurrealDayRepository::new().await;
    let exercise_repo = SurrealExerciseRepository::new().await;
    let db = establish_connection().await;
    if env::var(string!("PREFILL")).unwrap().eq_ignore_ascii_case("true") {
        db.query(r#"
            remove table if exists exercise;
            remove table if exists day;
            remove table if exists routine;
            remove table if exists gym;
        "#).await.unwrap();
        if let Ok(gym_name) = env::var(string!("GYM_NAME")) {
            db.query(r#"
                INSERT INTO gym {
                    nombre: $gym_name
                }"#)
                .bind(("gym_name",gym_name)).await.unwrap();

        }
    }
    let app_data = Data::new(routine_repo);
    println!("Starting...");

    let app = HttpServer::new(move || {
        let cors = Cors::default().allowed_origin(&env::var(string!("ORIGIN")).unwrap());
        let cors = match &env::var(string!("ORIGIN_SECOND")) {
            Ok(var) => cors
                .allowed_origin(var)
                .allow_any_method()
                .allow_any_header()
                .max_age(None),
            Err(_) => cors.allow_any_method().allow_any_header().max_age(None),
        };

        App::new()
            .app_data(app_data.to_owned())
            .app_data(Data::new(day_repo.to_owned()))
            .app_data(Data::new(exercise_repo.to_owned()))
            .app_data(Data::new(db.to_owned()))
            .wrap(Logger::default())
            .wrap(cors)
            .configure(|config| root_routes(config))
    })
        .bind((env::var(string!("HOST")).expect("HOST not set").as_str(), 8088))?;
    println!("Running!");
    app.run().await
}
