use specs::{Component, VecStorage};

#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct Speed {
    pub x_speed: f32,
    pub y_speed: f32,
}

impl Speed {
    pub fn new(x_speed: f32, y_speed: f32) -> Self {
        Self { x_speed, y_speed }
    }
}

#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct MaxSpeed {
    pub max_x_speed: f32,
    pub max_y_speed: f32,
}

impl MaxSpeed {
    pub fn new(max_x_speed: f32, max_y_speed: f32) -> Self {
        Self {
            max_x_speed,
            max_y_speed,
        }
    }
}
