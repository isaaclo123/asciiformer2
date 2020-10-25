extern crate euclid;
extern crate lazy_static;
extern crate specs;
extern crate termion;

mod components;
mod consts;
mod io;
mod resources;
mod systems;
mod utils;

use specs::{Builder, DispatcherBuilder, World, WorldExt};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use systems::{Keyboard, Physics, Renderer};
use termion::input::TermRead;
use utils::Direction;

use components::{
    BulletTextures, Color, ColorType, KeyboardControlled, PlayerTextures, Position, Texture,
    Velocity,
};
use io::get_stdin;
use resources::Map;
use termion::event::*;

fn main() {
    let mut world = World::new();

    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Texture>();
    world.register::<Color>();
    world.register::<KeyboardControlled>();

    let mut dispatcher = DispatcherBuilder::new()
        .with(Keyboard, "Keyboard", &[])
        .with(Physics, "Physics", &["Keyboard"])
        .with(Renderer::new(0, 0), "Renderer", &["Physics"])
        .build();

    dispatcher.setup(&mut world);

    let map = Map::new("map1.txt").expect("Unable to load map");
    world.insert(map);

    let movements: Arc<RwLock<Vec<Direction>>> = Arc::new(RwLock::new(vec![]));
    world.insert(Arc::clone(&movements));

    // world
    //     .create_entity()
    //     .with(Position::new(1.6, 1.6))
    //     .with(Velocity::new(1.1, 1.1))
    //     .with(Texture::new(&PlayerTextures))
    //     .with(Color::new(ColorType::Magenta))
    //     .build();

    // world
    //     .create_entity()
    //     .with(Position::new(0.6, 0.6))
    //     .with(Velocity::new(1.1, 1.1))
    //     .with(Texture::new(&PlayerTextures))
    //     .with(Color::new(ColorType::Red))
    //     .build();

    world
        .create_entity()
        .with(Position::new(10.6, 10.6))
        .with(Velocity::new(1.1, 1.1))
        .with(Texture::new(&PlayerTextures))
        .with(Color::new(ColorType::Blue))
        .with(KeyboardControlled)
        .build();

    // world
    //     .create_entity()
    //     .with(Position::new(15.0, 15.0))
    //     .with(Velocity::new(1.1, 1.1))
    //     .with(Texture::new(&BulletTextures))
    //     .with(Color::new(ColorType::Green))
    //     .build();

    // world
    //     .create_entity()
    //     .with(Position::new(14.0, 18.0))
    //     .with(Velocity::new(1.1, 1.1))
    //     .with(Texture::new(&BulletTextures))
    //     .build();

    let mut i = 0;
    // let mut run = true;

    dispatcher.dispatch(&mut world);
    world.maintain();

    let movements_cloned = Arc::clone(&movements);

    thread::spawn(move || {
        for c in get_stdin().events() {
            let mut mvmt = movements_cloned.write().unwrap();
            match c.unwrap() {
                Event::Key(ke) => match ke {
                    // Key::Char('q') => {
                    //     &run = false;
                    // }
                    Key::Char('w') | Key::Char(' ') => {
                        mvmt.push(Direction::Up);
                    }
                    Key::Char('a') => {
                        mvmt.push(Direction::Left);
                    }
                    Key::Char('s') => {
                        mvmt.push(Direction::Down);
                    }
                    Key::Char('d') => {
                        mvmt.push(Direction::Up);
                    }
                    _ => (),
                },
                _ => (),
            };
        }
    });

    loop {
        // Handle events

        *world.write_resource() = Arc::clone(&movements);

        // Update
        i = (i + 1) % 255;

        // Render
        dispatcher.dispatch(&mut world);
        world.maintain();

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
