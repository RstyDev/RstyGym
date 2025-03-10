use thiserror::Error;

pub type AppRes<T> = std::result::Result<T, AppError>;
pub type StrRes<T> = std::result::Result<T, String>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Index out of bounds \nLine {0}")]
    IndexErr(u16),
    #[error("DB Error: {1} \nLine {0}")]
    DBErr(u16,String),
    #[error("Unknown state: {1} \nLine {0}")]
    UnknownState(u16,String),
    #[error("No current routine \nLine {0}")]
    NoCurrentRoutine(u16),
}
