use crate::components::{Position, Velocity};
use euclid::default::Vector2D;
use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut position, velocity): Self::SystemData) {
        for (pos, vel) in (&mut position, &velocity).join() {
            pos.0 = pos.0 + vel.0
        }
    }
}
