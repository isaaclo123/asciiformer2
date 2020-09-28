#[macro_use]
extern crate vector2math;
extern crate lazy_static;
extern crate specs;
extern crate termion;

mod components;
mod consts;
mod io;
mod systems;

use specs::{Builder, RunNow, World, WorldExt};
use systems::Renderer;

use components::{Position, Velocity};
use vector2math::Vector2;

fn main() {
    let mut world = World::new();

    world.register::<Position>();
    world.register::<Velocity>();

    world
        .create_entity()
        .with(Position::new(0.0, 0.0))
        .with(Velocity::new(1.1, 1.1))
        .build();

    let mut renderer = Renderer;
    renderer.run_now(&world);
    world.maintain();
}
