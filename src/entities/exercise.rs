use serde::{Serialize, Deserialize};
#[cfg(feature = "ssr")]
use surrealdb::RecordId;
#[cfg(feature = "ssr")]
use crate::backend::infrastructure::db::ExerciseDB;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Exercise {
    id: Option<String>,
    name: String,
    series: [Option<Series>; 4],
    group: MuscleGroup,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Default)]
pub enum MuscleGroup {
    #[default]
    Chest,
    Back,
    Shoulders,
    Biceps,
    Triceps,
    Forearms,
    Legs,
    Abs,
    LowerBack,
}


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Series {
    count: u8,
    weight: Option<f32>,
}
impl Series {
    pub fn new(count: u8, weight: Option<f32>) -> Self {
        Self { count, weight }
    }
}
impl Exercise {
    pub fn new(id: Option<String>, name: String, series: [Option<Series>; 4], group: MuscleGroup) -> Self {
        Self { id, name, series, group }
    }
    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }
    #[cfg(feature = "ssr")]
    pub fn record(&self) -> Option<RecordId> {
        self.id.as_ref().map(|id|RecordId::from(("exercises",id)))
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
