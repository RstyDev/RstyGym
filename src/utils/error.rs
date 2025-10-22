#[cfg(feature = "ssr")]
use actix_web::{HttpResponse, Responder};
#[cfg(feature = "ssr")]
use actix_web::HttpRequest;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type AppRes<T> = std::result::Result<T, AppError>;
pub type StrRes<T> = std::result::Result<T, String>;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum AppError {
    #[error("Index out of bounds \nLine {0}")]
    IndexErr(u16),
    #[error("DB Error: {1} \nLine {0}")]
    DBErr(u16, String),
    #[error("Unknown state: {1} \nLine {0}")]
    UnknownState(u16, String),
    #[error("No current routine \nLine {0}")]
    NotFound(u16),
    #[error("Validation error: {1} \nLine{0}")]
    ValidationErr(u16, String),
    #[error("HTTP error: {1} \nLine{0}")]
    HttpErr(u16, String),
}



#[cfg(feature = "ssr")]
impl AppError{
    pub fn to_response(self) -> HttpResponse {
        match self{
            AppError::IndexErr(e) => {
                HttpResponse::InternalServerError().json(e)
            }
            AppError::DBErr(line, e) => {
                HttpResponse::InternalServerError().json((line,e))
            }
            AppError::UnknownState(line, e) => {
                HttpResponse::InternalServerError().json((line,e))
            }
            AppError::NotFound(e) => {
                HttpResponse::NotFound().json(e)
            }
            AppError::ValidationErr(line, e) => {
                HttpResponse::InternalServerError().json((line,e))
            }
            AppError::HttpErr(line, e) => {
                HttpResponse::InternalServerError().json((line,e))
            }
        }
    }
}

#[cfg(feature = "ssr")]
impl From<surrealdb::Error> for AppError {
    fn from(value: surrealdb::Error) -> Self {
        AppError::DBErr(59, value.to_string())
    }
}