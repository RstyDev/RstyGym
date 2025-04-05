use serde::Serialize;
use structs::routine::Routine;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewRoutine {
    pub name: String,
    pub routine: Routine,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWeight {
    pub exercise_index: u8,
    pub index: u8,
    pub weight: f32
}