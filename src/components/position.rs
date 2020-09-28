use specs::{Component, VecStorage};
use vector2math::*;

#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {
    x: f32,
    y: f32,
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
