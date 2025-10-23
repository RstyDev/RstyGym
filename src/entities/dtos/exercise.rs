use serde::{Deserialize, Serialize};
use crate::entities::{Exercise, MuscleGroup, Series};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ExerciseDTO {
    pub device: String,
    pub id: Option<String>,
    pub name: String,
    pub series: [Option<Series>; 4],
    pub group: MuscleGroup,
}

impl ExerciseDTO {
    pub fn into_inner(self)-> (String,Exercise) {
        (self.device, Exercise::new(self.id,self.name,self.series,self.group))
    }
}