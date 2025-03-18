#![allow(non_snake_case)]
use serde::Serialize;
use structs::routine::Routine;

#[derive(Serialize)]
pub struct NewRoutine {
    pub name: String,
    pub routine: Routine,
}
#[derive(Serialize)]
pub struct UpdateWeight {
    pub exerciseIndex: u8,
    pub index: u8,
    pub weight: f32
}