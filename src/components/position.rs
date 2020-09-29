use specs::{Component, VecStorage};
use vector2math::*;

#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {
    x: f32,
    y: f32,
}

impl Position {
    pub fn floor(self) -> (i16, i16) {
        (self.x.floor() as i16, self.y.floor() as i16)
    }

    pub fn ceil(self) -> (i16, i16) {
        (self.x.ceil() as i16, self.y.ceil() as i16)
    }
}

impl Vector2 for Position {
    type Scalar = f32;

    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    fn x(self) -> f32 {
        self.x
    }
    fn y(self) -> f32 {
        self.y
    }
}
