use std::sync::Rc;

pub struct Exercise {
    id: i64,
    name: Rc<str>,

}
pub struct Series {
    id: i64,
    count: u8,
    weight: Option<f32>,
}

impl Series{
    pub fn build(id: Option<i64>, count: u8, weight: Option<f32>)->Self{
        Self{ id: id.unwrap_or_default(), count, weight }
    }
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn set_id(&mut self, id: i64) {
        self.id = id;
    }
    pub fn count(&self) -> u8 {
        self.count
    }
    pub fn set_count(&mut self, count: u8) {
        self.count = count;
    }
    pub fn weight(&self) -> Option<f32> {
        self.weight
    }
    pub fn set_weight(&mut self, weight: f32){
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