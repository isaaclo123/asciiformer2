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
use std::time::Duration;
use systems::Renderer;
use termion::input::TermRead;

use components::{BulletTextures, Color, ColorType, PlayerTextures, Position, Texture, Velocity};
use io::{get_stdin, STDIN};
use resources::Map;
use std::sync::{Arc, Mutex};
use termion::async_stdin;
use termion::event::*;
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

    let mut i = 0;

    renderer.run_now(&world);
    world.maintain();

    'running: loop {
        // Handle events

        for c in get_stdin().events() {
            let result = match c.unwrap() {
                Event::Key(ke) => match ke {
                    Key::Char('q') => {
                        break 'running;
                    }
                    _ => (),
                },
                _ => (),
            };
        }

        // Update
        i = (i + 1) % 255;

        // Render
        renderer.run_now(&world);
        world.maintain();

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000u32 / 60));
    }
}
