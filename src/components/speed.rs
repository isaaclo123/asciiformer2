use specs::{Component, VecStorage};

#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct Speed {
    x_speed: f32,
    y_speed: f32,
}

impl Speed {
    pub fn new(x_speed: f32, y_speed: f32) -> Self {
        Self { x_speed, y_speed }
    }

    pub fn get_x_speed(self) -> f32 {
        self.x_speed
    }

    pub fn get_y_speed(self) -> f32 {
        self.y_speed
    }
}

#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct MaxSpeed {
    max_x_speed: f32,
    max_y_speed: f32,
}

impl MaxSpeed {
    pub fn new(max_x_speed: f32, max_y_speed: f32) -> Self {
        Self {
            max_x_speed,
            max_y_speed,
        }
    }

    pub fn get_max_x_speed(self) -> f32 {
        self.max_x_speed
    }

    pub fn get_max_y_speed(self) -> f32 {
        self.max_y_speed
    }
}
