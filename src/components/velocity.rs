use euclid::default::Vector2D;
use specs::{Component, VecStorage};

#[derive(Component, Debug, Default, Copy, Clone)]
#[storage(VecStorage)]
pub struct Velocity(pub Vector2D<f32>);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vector2D::new(x, y))
    }

    pub fn new_from_points(p0: Vector2D<f32>, p1: Vector2D<f32>, vel: f32) -> Self {
        let diff = p1 - p0;
        let magnitude = (diff.x * diff.x + diff.y * diff.y).sqrt();
        Self(diff * vel / magnitude)
    }
}
