use specs::{Component, VecStorage};

#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct Speed {
    speed: f32,
    max_speed: f32,
}

impl Speed {
    pub fn new(speed: f32, max_speed: f32) -> Self {
        Self { speed, max_speed }
    }

    pub fn get_speed(self) -> f32 {
        self.speed
    }

    pub fn get_max_speed(self) -> f32 {
        self.max_speed
    }
}
