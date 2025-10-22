use serde::{Deserialize, Serialize};
use crate::entities::DayTemplate;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct NewRoutineDTO {
    pub created_by: String,
    pub templates: Vec<DayTemplate>
}