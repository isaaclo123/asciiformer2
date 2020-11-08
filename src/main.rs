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

use euclid::default::Vector2D;
use specs::{Builder, DispatcherBuilder, World, WorldExt};
use std::io::Write;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};
use systems::{Keyboard, Physics, Renderer};
use termion::input::TermRead;
use termion::raw::RawTerminal;
use termion::{clear, cursor};
use utils::Direction;

use components::*;
// use components::{
//     BulletTextures, Color, ColorType, Friction, Gravity, KeyboardControlled, PlayerTextures,
//     Position, Texture, Velocity,
// };
use io::{get_stdin, get_stdout};
use resources::Map;
use termion::event::*;

fn main() {
    let mut world = World::new();

    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Texture>();
    world.register::<Color>();
    world.register::<Speed>();
    world.register::<Gravity>();
    world.register::<Friction>();
    world.register::<KeyboardControlled>();

    let mut dispatcher = DispatcherBuilder::new()
        .with(Keyboard, "Keyboard", &[])
        .with(Physics, "Physics", &["Keyboard"])
        .with(Renderer::new(), "Renderer", &["Physics"])
        .build();

    dispatcher.setup(&mut world);

    let map = Map::new("map1.txt").expect("Unable to load map");
    world.insert(map);

    let movements: Arc<RwLock<Vec<Direction>>> = Arc::new(RwLock::new(vec![]));
    world.insert(Arc::clone(&movements));

    let origin: Vector2D<u16> = Vector2D::new(0, 0);
    world.insert(origin);

    world
        .create_entity()
        .with(Position::new(1.0, 1.0))
        .with(Velocity::new(0.0, 0.0))
        .with(Texture::new(&PlayerTextures))
        .with(Color::new(ColorType::Red))
        .with(Speed::new(0.8, 0.5))
        .with(MaxSpeed::new(2.0, 3.0))
        .with(Gravity::new(0.1))
        .with(Friction::new(0.15))
        .with(MaxJump::new(1))
        .with(Collide::new(OnCollideType::Block))
        .with(KeyboardControlled)
        .build();

    // world
    //     .create_entity()
    //     .with(Position::new(15.0, 15.0))
    //     .with(Velocity::new(1.1, 1.1))
    //     .with(Texture::new(&BulletTextures))
    //     .with(Color::new(ColorType::Green))
    //     .build();

    let movements_cloned = Arc::clone(&movements);

    thread::spawn(move || {
        for c in get_stdin().events() {
            let mut mvmt = movements_cloned.write().unwrap();
            match c.unwrap() {
                Event::Mouse(me) => match me {
                    MouseEvent::Press(_, a, b) => {
                        let mouse_pt = Vector2D::new(a, b);
                        let int_pt = mouse_pt.cast::<i16>() - origin.cast::<i16>();
                        mvmt.push(Direction::To(int_pt));
                    }
                    _ => (),
                },
                Event::Key(ke) => match ke {
                    Key::Char('q') => {
                        RawTerminal::suspend_raw_mode(&get_stdout()).unwrap();
                        write!(get_stdout(), "{}{}", clear::All, cursor::Show).unwrap();
                        get_stdout().flush().unwrap();

                        std::process::exit(0)
                    }
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
                        mvmt.push(Direction::Right);
                    }
                    _ => (),
                },
                _ => (),
            };
        }
    });

    let mut before = Instant::now();
    let interval = 60;

    loop {
        // Render
        *world.write_resource() = Arc::clone(&movements);
        dispatcher.dispatch(&mut world);
        world.maintain();

        let now = Instant::now();
        let dt = (now.duration_since(before).subsec_nanos() / 1_000_000) as u64;

        if dt < interval {
            thread::sleep(Duration::from_millis(interval - dt));
            continue;
        }

        before = now;
    }
}
