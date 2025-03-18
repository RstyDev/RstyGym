use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Exercise {
    id: i64,
    name: String,
    series: [Option<Series>; 4],
    group: MuscleGroup,
}
impl Exercise {
    pub fn build(
        id: Option<i64>,
        name: String,
        series: [Option<Series>; 4],
        group: MuscleGroup,
    ) -> Exercise {
        Exercise {
            id: id.unwrap_or_default(),
            name,
            series,
            group,
        }
    }
    pub fn id(&self) -> &i64 {
        &self.id
    }
    pub fn set_id(&mut self, id: i64) {
        self.id = id;
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn series(&self) -> &[Option<Series>; 4] {
        &self.series
    }
    pub fn series_at(&self, index: usize) -> Option<&Series> {
        if index <= self.series.len() {
            self.series[index].as_ref()
        } else {
            None
        }
    }
    pub fn series_at_mut(&mut self, index: usize) -> Option<&mut Option<Series>> {
        if index <= self.series.len() {
            Some(&mut self.series[index])
        } else {
            None
        }
    }
    pub fn set_series(&mut self, series: [Option<Series>; 4]) {
        self.series = series;
    }
    pub fn set_series_at(&mut self, index: usize, series: Option<Series>) {
        self.series[index] = series;
    }
    pub fn group(&self) -> &MuscleGroup {
        &self.group
    }
    pub fn set_group(&mut self, group: MuscleGroup) {
        self.group = group;
    }
}
impl PartialEq for Exercise {
    fn eq(&self, other: &Self) -> bool {
        self.name == self.name
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Series {
    id: i64,
    count: u8,
    weight: Option<f32>,
}

impl Series {
    pub fn build(id: Option<i64>, count: u8, weight: Option<f32>) -> Self {
        Self {
            id: id.unwrap_or_default(),
            count,
            weight,
        }
    }
    pub fn id(&self) -> &i64 {
        &self.id
    }
    pub fn set_id(&mut self, id: i64) {
        self.id = id;
    }
    pub fn count(&self) -> &u8 {
        &self.count
    }
    pub fn set_count(&mut self, count: u8) {
        self.count = count;
    }
    pub fn weight(&self) -> Option<&f32> {
        self.weight.as_ref()
    }
    pub fn set_weight(&mut self, weight: f32) {
        self.weight = Some(weight);
    }
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

impl MuscleGroup {
    pub fn iter() -> [MuscleGroup; 9] {
        [
            MuscleGroup::Chest,
            MuscleGroup::Back,
            MuscleGroup::Shoulders,
            MuscleGroup::Biceps,
            MuscleGroup::Triceps,
            MuscleGroup::Forearms,
            MuscleGroup::Legs,
            MuscleGroup::Abs,
            MuscleGroup::LowerBack,
        ]
    }
}

impl Display for MuscleGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            match self {
                MuscleGroup::Chest => "Chest",
                MuscleGroup::Back => "Back",
                MuscleGroup::Shoulders => "Shoulders",
                MuscleGroup::Biceps => "Biceps",
                MuscleGroup::Triceps => "Triceps",
                MuscleGroup::Forearms => "Forearms",
                MuscleGroup::Legs => "Legs",
                MuscleGroup::Abs => "Abs",
                MuscleGroup::LowerBack => "LowerBack",
            }
        ))
    }
}

impl TryFrom<String> for MuscleGroup {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Chest" => Ok(MuscleGroup::Chest),
            "Back" => Ok(MuscleGroup::Back),
            "Shoulders" => Ok(MuscleGroup::Shoulders),
            "Biceps" => Ok(MuscleGroup::Biceps),
            "Triceps" => Ok(MuscleGroup::Triceps),
            "Forearms" => Ok(MuscleGroup::Forearms),
            "Legs" => Ok(MuscleGroup::Legs),
            "Abs" => Ok(MuscleGroup::Abs),
            "LowerBack" => Ok(MuscleGroup::LowerBack),
            _ => Err(()),
        }
    }
}
