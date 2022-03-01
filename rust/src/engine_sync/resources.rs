use gdrust::macros::single_value;

pub trait Delta {
    fn set_delta(&mut self, delta: f32);
}

#[single_value(extends = f32)]
#[derive(Default)]
pub struct IdleDelta {
    pub value: f32,
}

impl Delta for IdleDelta {
    fn set_delta(&mut self, delta: f32) {
        self.value = delta;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GameOver {
    Win,
    Lose,
}

#[single_value(extends = f32)]
#[derive(Default)]
pub struct PhysicsDelta {
    pub value: f32,
}

impl Delta for PhysicsDelta {
    fn set_delta(&mut self, delta: f32) {
        self.value = delta;
    }
}
