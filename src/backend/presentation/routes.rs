// use crate::backend::presentation::handlers::{all_families, change_password, delete_book, delete_family, family_by_id, get_book_by_id, register_book, register_family, update_book, update_family};
// use crate::backend::presentation::handlers::{
//     all_users, delete_user, register_user_handler, update_user, user_by_id, get_all_books
// };
// use crate::backend::presentation::handlers::{login, refresh_token, validator};
use actix_cors::Cors;
use actix_web::web;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use std::env;
use crate::backend::presentation::handlers::{validator, login, refresh_token, register, save_exercise, get_exercises};

pub fn root_routes(config: &mut web::ServiceConfig) {
    let (auth, auth2,auth3) = (
        HttpAuthentication::with_fn(move |a, b: Option<BearerAuth>| validator(a, b)),
        HttpAuthentication::with_fn(move |a, b: Option<BearerAuth>| validator(a, b)),
        HttpAuthentication::with_fn(move |a, b: Option<BearerAuth>| validator(a, b)),
    );
    let origin = env::var(String::from("ORIGIN")).unwrap();
    let (cors, cors2, cors3) = (
        Cors::default().allowed_origin(&origin),
        Cors::default().allowed_origin(&origin),
        Cors::default().allowed_origin(&origin),
    );
    let (cors, cors2,cors3) = match &env::var(String::from("ORIGIN_SECOND")) {
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
                // .service(all_families)
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
