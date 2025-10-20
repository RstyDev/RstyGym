use serde::{Serialize, Deserialize};
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
impl Exercise {
    pub fn id(&self) -> &Option<String> {
        &self.id
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