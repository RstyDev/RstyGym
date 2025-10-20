use crate::backend::infrastructure::db::establish_connection;
use crate::backend::infrastructure::prefill::prefill;
use crate::backend::infrastructure::repositories::{SurrealBookRepository, SurrealFamilyRepository};
use crate::backend::{
    infrastructure::repositories::SurrealUserRepository, presentation::routes::root_routes,
};
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

pub async fn run() -> std::io::Result<()> {
    dotenv().ok();
    let repo = SurrealUserRepository::new().await;
    let family_repo = SurrealFamilyRepository::new().await;
    let book_repo = SurrealBookRepository::new().await;
    let db = establish_connection().await;
    if env::var("PREFILL").unwrap().eq_ignore_ascii_case("true") {
        db.query(r#"
            remove table if exists exercise;
            remove table if exists day;
            remove table if exists routine;
            // remove table if exists familias;
        "#).await.unwrap();
        prefill(Arc::from(repo.clone()), Arc::from(book_repo.clone()), Arc::from(family_repo.clone())).await;
        if let Ok(church_name) = env::var("CHURCH_NAME") {
            let denomination = env::var("DENOMINATION").unwrap();
            let presbytery = env::var("PRESBYTERY").unwrap();
            db.query(r#"
                INSERT INTO iglesia {
                    nombre: $church_name,
                    denominacion: $denomination,
                    presbiterio: $presbytery
                }"#)
                .bind(("church_name",church_name))
                .bind(("denomination",denomination))
                .bind(("presbytery",presbytery)).await.unwrap();

        }
    }
    let app_data = Data::new(repo);
    println!("Starting...");

    let app = HttpServer::new(move || {
        let cors = Cors::default().allowed_origin(&env::var("ORIGIN").unwrap());
        let cors = match &env::var("ORIGIN_SECOND") {
            Ok(var) => cors
                .allowed_origin(var)
                .allow_any_method()
                .allow_any_header()
                .max_age(None),
            Err(_) => cors.allow_any_method().allow_any_header().max_age(None),
        };

        App::new()
            .app_data(app_data.to_owned())
            .app_data(Data::new(family_repo.to_owned()))
            .app_data(Data::new(book_repo.to_owned()))
            .app_data(Data::new(db.to_owned()))
            .wrap(Logger::default())
            .wrap(cors)
            .configure(|config| root_routes(config))
    })
        .bind((env::var("HOST").expect("HOST not set").as_str(), 8088))?;
    println!("Running!");
    app.run().await
}
