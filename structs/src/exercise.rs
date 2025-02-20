use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Exercise {
    id: i64,
    name: Rc<str>,
    series: [Option<Series>;4],
    group: MuscleGroup,
}
impl Exercise {
    pub fn build(id: Option<i64>, name: &str, series: [Option<Series>;4], group: MuscleGroup) -> Exercise {
        Exercise {
            id: id.unwrap_or_default(),
            name: name.into(),
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
    pub fn set_name(&mut self, name: &str) {
        self.name = name.into();
    }
    pub fn series(&self) -> &[Option<Series>;4] {
        &self.series
    }
    pub fn series_at(&self, index: usize) -> Option<&Series> {
        if index >= self.series.len() {
            self.series[index].as_ref()
        }else{None}
    }
    pub fn series_at_mut(&mut self, index: usize) -> Option<&mut Series> {
        if index >= self.series.len() {
            self.series[index].as_mut()
        }else{None}
    }
    pub fn set_series(&mut self, series: [Option<Series>;4]) {
        self.series = series;
    }
    pub fn group(&self) -> &MuscleGroup {
        &self.group
    }
    pub fn set_group(&mut self, group: MuscleGroup) {
        self.group = group;
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
