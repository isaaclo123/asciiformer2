use crate::components::{Position, Velocity};
use crate::io::*;
use specs::{Join, ReadStorage, System};
use std::io::Write;

pub struct Renderer;

impl<'a> System<'a> for Renderer {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (position, velocity): Self::SystemData) {
        for (pos, vel) in (&position, &velocity).join() {
            write!(get_stdout(), "{:?} {:?}", &pos, &vel).unwrap();
            // println!("{:?} {:?}", &pos, &vel);
        }
    }
}
