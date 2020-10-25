use euclid::default::Vector2D;
use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity(pub Vector2D<f32>);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vector2D::new(x, y))
    }
}
