use serde::{Serialize, Deserialize};
use surrealdb::RecordId;
use crate::entities::{Exercise, MuscleGroup, Series};
use crate::record_id;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ExerciseDB {
    id: Option<RecordId>,
    name: String,
    series: [Option<Series>; 4],
    group: MuscleGroup,
}

impl From<Exercise> for ExerciseDB {
    fn from(value: Exercise) -> Self {
        ExerciseDB::new(value.id().cloned(),value.name().to_owned(),value.series().to_owned(),value.group())
    }
}


impl ExerciseDB {
    pub fn new(id: Option<String>, name: String, series: [Option<Series>; 4], group: MuscleGroup) -> Self {
        let id = id.map(|id|record_id!("exercises",id));
        Self { id, name, series, group }
    }
    pub fn build(exercise: Exercise) -> Self {
        Self {
            id: exercise.record(),
            name: exercise.name().into(),
            series: exercise.series().to_owned(),
            group: Default::default(),
        }
    }

    pub fn id(&self) -> Option<&RecordId> {
        self.id.as_ref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn series(&self) -> &[Option<Series>; 4] {
        &self.series
    }

    pub fn group(&self) -> MuscleGroup {
        self.group
    }

}