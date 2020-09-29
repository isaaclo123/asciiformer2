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

use components::{Color, ColorType, PlayerTextures, Position, Texture, Velocity};
use vector2math::Vector2;

fn main() {
    let mut world = World::new();

    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Texture>();
    world.register::<Color>();

    world
        .create_entity()
        .with(Position::new(0.6, 0.6))
        .with(Velocity::new(1.1, 1.1))
        .with(Texture::new(&PlayerTextures))
        .with(Color::new(ColorType::Red))
        .build();

    world
        .create_entity()
        .with(Position::new(10.6, 10.6))
        .with(Velocity::new(1.1, 1.1))
        .with(Texture::new(&PlayerTextures))
        .with(Color::new(ColorType::Blue))
        .build();

    let mut renderer = Renderer::new(0, 0);
    renderer.run_now(&world);
    world.maintain();
}
