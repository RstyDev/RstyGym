use thiserror::Error;

pub type AppRes<T> = std::result::Result<T, AppError>;
pub type StrRes<T> = std::result::Result<T, String>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Index out of bounds")]
    IndexErr,
    #[error("DB Error: {0}")]
    DBErr(String),
    #[error("Unknown state: {0}")]
    UnknownState(String)
}
