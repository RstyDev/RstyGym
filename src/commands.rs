use serde::Serialize;
use structs::routine::Routine;

#[derive(Serialize)]
pub struct NewRoutine {
    pub name: String,
    pub routine: Routine,
}