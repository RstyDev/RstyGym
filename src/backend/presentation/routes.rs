use actix_cors::Cors;
use actix_web::web;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use std::env;
use crate::{string,backend::presentation::handlers::{
    validator, login, refresh_token, register, save_exercise, get_exercises, save_day, get_days,
    save_routine, get_routines, update_exercises,
}};

pub fn root_routes(config: &mut web::ServiceConfig) {
    let (auth, auth2,auth3) = (
        HttpAuthentication::with_fn(move |a, b: Option<BearerAuth>| validator(a, b)),
        HttpAuthentication::with_fn(move |a, b: Option<BearerAuth>| validator(a, b)),
        HttpAuthentication::with_fn(move |a, b: Option<BearerAuth>| validator(a, b)),
    );
    let origin = env::var(string!("ORIGIN")).unwrap();
    let (cors, cors2, cors3) = (
        Cors::default().allowed_origin(&origin),
        Cors::default().allowed_origin(&origin),
        Cors::default().allowed_origin(&origin),
    );
    let (cors, cors2,cors3) = match &env::var(string!("ORIGIN_SECOND")) {
        Ok(var) => (
            cors.allowed_origin(var)
                .allow_any_method()
                .allow_any_header()
                .max_age(None),
            cors2
                .allowed_origin(var)
                .allow_any_method()
                .allow_any_header()
                .max_age(None),
            cors3
                .allowed_origin(var)
                .allow_any_method()
                .allow_any_header()
                .max_age(None),
        ),
        Err(_) => (
            cors.allow_any_method().allow_any_header().max_age(None),
            cors2.allow_any_method().allow_any_header().max_age(None),
            cors3.allow_any_method().allow_any_header().max_age(None),
        ),
    };

    config
        .service(login)
        .service(refresh_token)
        .service(register)
        .service(
            web::scope("/routines")
                .wrap(cors)
                .wrap(auth)
                .service(save_routine)
                .service(get_routines)
                // .service(register_user_handler)
                // .service(all_users)
                // .service(user_by_id)
                // .service(delete_user)
                // .service(update_user)
                // .service(change_password),
        )
        .service(
            web::scope("/days")
                .wrap(cors2)
                .wrap(auth2)
                .service(save_day)
                .service(get_days)
                .service(update_exercises)
                // .service(delete_family)
                // .service(family_by_id)
                // .service(register_family)
                // .service(update_family)
        )
        .service(web::scope("/exercises")
                     .wrap(cors3)
                     .wrap(auth3)
                     .service(save_exercise)
                     .service(get_exercises)
                     // .service(get_all_books)
                     // .service(get_book_by_id)
                     // .service(delete_book)
                     // .service(update_book)
                     // .service(register_book)
                 ,
                 /*
                 delete_family
                 get_all_families
                 get_family_by_id
                 save_family
                 update_family
                 */
        );
}
