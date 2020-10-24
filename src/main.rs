#[macro_use]
extern crate vector2math;
extern crate lazy_static;
extern crate specs;
extern crate termion;

mod components;
mod consts;
mod io;
mod resources;
mod systems;

use specs::{Builder, RunNow, World, WorldExt};
use systems::Renderer;

use components::{BulletTextures, Color, ColorType, PlayerTextures, Position, Texture, Velocity};
use resources::Map;
use vector2math::Vector2;

fn main() {
    let mut world = World::new();

    world.insert(Map::new("map1.txt").unwrap());

    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Texture>();
    world.register::<Color>();

    let map = Map::new("map1.txt").expect("Unable to load map");

    world.insert(map);

    world
        .create_entity()
        .with(Position::new(1.6, 1.6))
        .with(Velocity::new(1.1, 1.1))
        .with(Texture::new(&PlayerTextures))
        .with(Color::new(ColorType::Magenta))
        .build();

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

    world
        .create_entity()
        .with(Position::new(15.0, 15.0))
        .with(Velocity::new(1.1, 1.1))
        .with(Texture::new(&BulletTextures))
        .with(Color::new(ColorType::Green))
        .build();

    world
        .create_entity()
        .with(Position::new(14.0, 18.0))
        .with(Velocity::new(1.1, 1.1))
        .with(Texture::new(&BulletTextures))
        .build();

    let mut renderer = Renderer::new(0, 0);
    renderer.run_now(&world);
    world.maintain();
}
